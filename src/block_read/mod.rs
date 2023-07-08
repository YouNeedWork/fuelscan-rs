use fuel_core_client::client::FuelClient;
use tokio::select;
use tokio::sync::broadcast;
use tracing::info;

#[derive(Debug)]
pub struct BlockReader {
    client: FuelClient,
    shutdown: broadcast::Receiver<()>,
}

pub enum BlockReaderError {
    ReadFromRpcError,
}

impl BlockReader {
    pub fn new(client: FuelClient, shutdown: broadcast::Receiver<()>) -> Self {
        Self { client, shutdown }
    }

    pub async fn start(&mut self) -> Result<(), BlockReaderError> {
        let mut height = 0;
        loop {
            let block = match self
                .client
                .block_by_height(height)
                .await
                .map_err(|_| BlockReaderError::ReadFromRpcError)?
            {
                Some(block) => block,
                None => {
                    info!("no block at height {}", height);
                    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                    continue;
                }
            };

            let header = block.header;

            info!(
                "block at height {} has {} txs",
                height,
                block.transactions.len()
            );

            let client = self.client.clone();

            let txs = block
                .transactions
                .iter()
                .map(|tx| tx.id.to_string())
                .into_iter()
                .map(|tx_hash| {
                    let id = tx_hash.clone().to_string().as_str();
                    let feat = client.transaction(id);

                    feat
                })
                .collect::<Vec<_>>();

            let txs = futures::future::join_all(txs).await;
            for tx in txs {
                if let Some(tx) = tx.map_err(|_| BlockReaderError::ReadFromRpcError)? {
                    info!("tx: {:?}", tx);
                }
            }

            height += 1;

            select! {
                _ = self.shutdown.recv() => {
                    info!("shutdown signal received");
                    return Ok(());
                }
            }
        }
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
