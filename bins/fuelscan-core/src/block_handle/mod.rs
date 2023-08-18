use crate::block_read::{BlockBodies, FetchBlockResult};

use fuel_core_client::client::schema::block::Header;

use models::PgSqlPool;

use crate::block_handle::process::process;
use std::time::Duration;
use thiserror::Error;
use tokio::{select, sync::broadcast};
use tracing::info;

use self::{blocks::insert_header, transactions::insert_tx};

pub mod blocks;
pub mod process;
pub mod transactions;

#[derive(Clone)]
pub struct BlockHandler {
    db_client: PgSqlPool,
    block_rx: flume::Receiver<Vec<FetchBlockResult>>,
    shutdown: broadcast::Sender<()>,
}

#[derive(Debug, Error)]
pub enum BlockHandlerError {
    #[error("failed to insert header into db: {0}")]
    InsertHeaderDb(String),
    #[error("failed to insert transaction into db: {0}")]
    InsertTransactionDb(String),
    #[error("failed to update check_point: {0}")]
    InsertUpdateCheckPoint(String),
    #[error("failed to insert into db: {0}")]
    InsertDb(String),
    #[error("failed to serialize json: {0}")]
    SerdeJson(String),
    #[error("process data error: {0}")]
    DataProcessError(String),
}

impl BlockHandler {
    pub fn new(
        db_client: PgSqlPool,
        block_rx: flume::Receiver<Vec<FetchBlockResult>>,
        shutdown: broadcast::Sender<()>,
    ) -> Self {
        Self {
            db_client,
            block_rx,
            shutdown,
        }
    }

    async fn insert_header_and_txs(
        &mut self,
        header: &Header,
        bodies: &BlockBodies,
    ) -> Result<(), BlockHandlerError> {
        let mut conn = self
            .db_client
            .get()
            .map_err(|e| BlockHandlerError::InsertDb(e.to_string()))?;

        let () = process(header, bodies)
            .await
            .map_err(|e| BlockHandlerError::DataProcessError(e.to_string()))?;

        insert_header(&mut conn, header).map_err(|e| BlockHandlerError::InsertDb(e.to_string()))?;
        /*     for tx in transactions {
                   insert_tx(header, tx).map_err(|e| BlockHandlerError::InsertDb(e.to_string()))?;
               }
        */
        Ok(())
    }

    pub async fn start(&mut self) -> Result<(), BlockHandlerError> {
        let mut shutdown = self.shutdown.subscribe();

        loop {
            select! {
                Ok(blocks) = self.block_rx.recv_async() => {
                    for (header, transactions) in blocks {
                        while let Err(_) = self.insert_header_and_txs(&header,&transactions).await {
                            info!("insert_header_and_tx failed, retrying");
                            tokio::time::sleep(Duration::from_secs(1)).await;
                        }
                    }
                }
                _ = shutdown.recv() => {
                    info!("BlockHandler shutdown");
                    return Ok(());
                }
            }
        }
    }
}

impl Drop for BlockHandler {
    fn drop(&mut self) {
        info!("BlockHandler drop");
    }
}
