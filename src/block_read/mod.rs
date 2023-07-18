use std::collections::HashMap;

use fuel_core_client::client::schema::block::Header;
use fuel_core_client::client::types::TransactionResponse;
use fuel_core_client::client::FuelClient;

use rusoto_dynamodb::{AttributeValue, DynamoDb, DynamoDbClient, GetItemInput};
use thiserror::Error;

use tokio::sync::mpsc;
use tracing::{info, trace};

pub type BlockMsg = Vec<(Header, Vec<(String, TransactionResponse)>)>;

pub struct BlockReader {
    batch_fetch_size: u64,
    client: FuelClient,
    db_client: DynamoDbClient,
    block_handler: mpsc::UnboundedSender<BlockMsg>,
}

#[derive(Error, Debug)]
pub enum BlockReaderError {
    #[error("The latest height block: {0}")]
    HeightBlock(u64),
    #[error("Read block info from rpc failed: {0}")]
    ReadFromRpc(String),
    #[error("Sender failed the Handler channel maybe closed: {0}")]
    SendToHandler(String),
}

impl BlockReader {
    pub fn new(
        batch_fetch_size: u64,
        client: FuelClient,
        db_client: DynamoDbClient,
        block_handler: mpsc::UnboundedSender<BlockMsg>,
    ) -> Self {
        Self {
            batch_fetch_size,
            client,
            db_client,
            block_handler,
        }
    }

    pub async fn start(&mut self) -> Result<(), BlockReaderError> {
        let db_client = self.db_client.clone();
        let mut height: u64 = async move {
            let input = GetItemInput {
                table_name: "check_point".to_string(),
                key: {
                    let mut key = HashMap::new();
                    key.insert(
                        "id".to_string(),
                        AttributeValue {
                            n: Some("1".to_string()),
                            ..Default::default()
                        },
                    );
                    key.insert(
                        "chain".to_string(),
                        AttributeValue {
                            n: Some("1".to_string()),
                            ..Default::default()
                        },
                    );
                    key
                },
                ..Default::default()
            };
            let get_res = db_client.get_item(input).await;

            match get_res {
                Ok(res) => match res.item {
                    Some(item) => {
                        let check_point = item
                            .get("check_point")
                            .expect("failed to get check_point")
                            .n
                            .as_ref()
                            .expect("failed to get check_point")
                            .parse::<u64>()
                            .expect("failed to parse height");
                        info!("check point height: {}", check_point);
                        check_point
                    }
                    None => 0,
                },
                Err(_) => 0,
            }
        }
        .await;

        loop {
            let fetch_feat = (height..(height + self.batch_fetch_size))
                .map(|h| Self::fetch_block(&self.client, h))
                .collect::<Vec<_>>();

            let maybe_blocks = futures::future::join_all(fetch_feat).await;
            let mut blocks = vec![];
            for block in maybe_blocks {
                match block {
                    Ok(b) => blocks.push(b),
                    Err(e) => info!("{}", e),
                }
            }

            height += blocks.len() as u64;

            self.block_handler
                .send(blocks)
                .map_err(|e| BlockReaderError::SendToHandler(e.to_string()))?;

            info!("Indexer Height {} wait for 5 secs", height);

            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        }
    }

    async fn fetch_block(
        client: &FuelClient,
        height: u64,
    ) -> Result<(Header, Vec<(String, TransactionResponse)>), BlockReaderError> {
        let block = match client
            .block_by_height(height)
            .await
            .map_err(|e| BlockReaderError::ReadFromRpc(e.to_string()))?
        {
            Some(block) => block,
            None => {
                trace!("no block at height {}", height);

                return Err(BlockReaderError::HeightBlock(height));
            }
        };

        let header = block.header;

        trace!(
            "block at height {} has {} txs",
            height,
            block.transactions.len()
        );

        let txs = block
            .transactions
            .iter()
            .map(|tx_hash| async move {
                let feat = client
                    .transaction(&tx_hash.id.to_string())
                    .await
                    .map_err(|e| BlockReaderError::ReadFromRpc(e.to_string()));

                (feat, tx_hash.id.to_string())
            })
            .collect::<Vec<_>>();
        let mut transactions = vec![];

        let maybe_empty_txs = futures::future::join_all(txs).await;
        for (tx, hash) in maybe_empty_txs {
            if let Some(tx) = tx.map_err(|e| BlockReaderError::ReadFromRpc(e.to_string()))? {
                transactions.push((hash, tx));
            }
        }

        Ok((header, transactions))
    }
}
