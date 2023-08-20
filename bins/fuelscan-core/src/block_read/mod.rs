use fuel_core_client::client::schema::block::Header;
use fuel_core_client::client::types::TransactionResponse;
use fuel_core_client::client::FuelClient;

use fuel_core_types::fuel_tx::Receipt;
use thiserror::Error;
use tracing::{error, info, trace};
pub type BlockBody = (String, Option<TransactionResponse>, Vec<Receipt>);

pub type BlockBodies = Vec<(String, Option<TransactionResponse>, Vec<Receipt>)>;

pub type FetchBlockResult = (Header, BlockBodies);

pub struct BlockReader {
    batch_fetch_size: u64,
    client: FuelClient,
    block_handler: flume::Sender<Vec<FetchBlockResult>>,
}

impl Drop for BlockReader {
    fn drop(&mut self) {
        info!("BlockReader drop");
    }
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
        block_handler: flume::Sender<Vec<FetchBlockResult>>,
    ) -> Self {
        Self {
            batch_fetch_size,
            client,
            block_handler,
        }
    }

    pub async fn start(&mut self, mut height: u64) -> Result<(), BlockReaderError> {
        loop {
            let fetch_feat = (height..(height + self.batch_fetch_size))
                .map(|h| Self::fetch_block(&self.client, h))
                .collect::<Vec<_>>();

            let maybe_blocks = futures::future::join_all(fetch_feat).await;
            let mut blocks = vec![];
            for block in maybe_blocks {
                match block {
                    Ok(block) => {
                        blocks.push(block);
                    }
                    Err(e) => {
                        error!("Got Rpc error {},retry in 2 secs", e);
                        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                        continue;
                    }
                }
            }

            height += blocks.len() as u64;

            self.block_handler
                .send(blocks)
                .map_err(|e| BlockReaderError::SendToHandler(e.to_string()))?;

            info!("Indexer Height {} wait for 2 secs", height);
            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        }
    }

    async fn fetch_block(
        client: &FuelClient,
        height: u64,
    ) -> Result<FetchBlockResult, BlockReaderError> {
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
                let reseipts = client
                    .receipts(&tx_hash.id.to_string())
                    .await
                    .map_err(|e| BlockReaderError::ReadFromRpc(e.to_string()));

                (feat, reseipts, tx_hash.id.to_string())
            })
            .collect::<Vec<_>>();
        let mut transactions = vec![];

        let maybe_empty_txs = futures::future::join_all(txs).await;
        for (tx, reseipts, hash) in maybe_empty_txs {
            transactions.push((hash, tx?, reseipts?));
        }

        Ok((header, transactions))
    }
}
