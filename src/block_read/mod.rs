use fuel_core_client::client::schema::{block::Header, schema::Transaction};
use fuel_core_client::client::types::TransactionResponse;
use fuel_core_client::client::FuelClient;

use thiserror::Error;

use tokio::sync::{broadcast, mpsc};
use tracing::{info, trace};

pub type BlockMsg = Vec<(Header, Vec<TransactionResponse>)>;

#[derive(Debug)]
pub struct BlockReader {
    batch_fetch_size: u64,
    client: FuelClient,
    block_handler: mpsc::Sender<BlockMsg>,
    shutdown: broadcast::Receiver<()>,
}

#[derive(Error, Debug)]
pub enum BlockReaderError {
    #[error("The latest height block: {0}")]
    HeightBlock(u64),
    #[error("Read block info from rpc failed: {0}")]
    ReadFromRpcError(String),
    #[error("Sender failed the Handler channel maybe closed: {0}")]
    SendToHandlerError(String),
}

impl BlockReader {
    pub fn new(
        batch_fetch_size: u64,
        client: FuelClient,
        shutdown: broadcast::Receiver<()>,
        block_handler: mpsc::Sender<BlockMsg>,
    ) -> Self {
        Self {
            batch_fetch_size,
            client,
            shutdown,
            block_handler,
        }
    }

    pub async fn start(&mut self) -> Result<(), BlockReaderError> {
        let mut height: u64 = 0;
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
                .await
                .map_err(|e| BlockReaderError::SendToHandlerError(e.to_string()))?;

            if height % 500 == 0 {
                info!("Indexer {}", height);
            }

            /*             select! {
                _ = self.shutdown.recv() => {
                    info!("shutdown signal received");
                    return Ok(());
                }
            } */
        }
    }

    async fn fetch_block(
        client: &FuelClient,
        height: u64,
    ) -> Result<(Header, Vec<TransactionResponse>), BlockReaderError> {
        let block = match client
            .block_by_height(height)
            .await
            .map_err(|e| BlockReaderError::ReadFromRpcError(e.to_string()))?
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
            .map(|tx_hash| {
                let feat = client.transaction(tx_hash.id.clone());

                feat
            })
            .collect::<Vec<_>>();
        let mut transactions = vec![];

        let maybe_empty_txs = futures::future::join_all(txs).await;
        for tx in maybe_empty_txs {
            if let Some(tx) = tx.map_err(|e| BlockReaderError::ReadFromRpcError(e.to_string()))? {
                //trace!("tx: {:?}", tx);
                //dbg!(&tx);
                transactions.push(tx);
            }
        }

        Ok((header, transactions))
    }
}

/* let client =
           FuelClient::from_str("https://beta-3.fuel.network").expect("failed to create client");

       let block = self
           .client
           .block_by_height(self.height)
           .await
           .expect("failed to get chain info")
           .unwrap();

       println!("{:?}", block.header.height);
       for tx in block.transactions {
           let full_tx = client
               .transaction(&tx.id.to_string())
               .await
               .expect("failed to get tx")
               .unwrap();

           dbg!(full_tx);
           //println!("{:?}", full_tx);
       }
*/
