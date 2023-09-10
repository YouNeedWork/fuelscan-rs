use crate::block_read::{BlockBodies, Blocks};

use fuel_core_client::client::types::block::Header;
use models::{
    block::batch_insert_block, call::batch_insert_calls, coinbase::batch_insert_coinbase,
    contract::batch_insert_contracts, transaction::batch_insert_transactions, PgSqlPool,
};

use crate::block_handle::process::process;
use std::time::Duration;
use thiserror::Error;
use tokio::{select, sync::broadcast};
use tracing::info;
pub mod assets;
pub mod blocks;
pub mod process;

pub type BlockHandleResult = Result<(), BlockHandlerError>;

#[derive(Debug, Error)]
pub enum BlockHandlerError {
    #[error("failed to insert header into db: {0}")]
    InsertHeaderDb(String),
    #[error("failed to insert transaction into db: {0}")]
    InsertTransactionDb(String),
    #[error("insert contracts failed: {0}")]
    InsertContract(String),
    #[error("insert calls failed: {0}")]
    InsertCalls(String),
    #[error("failed to insert into db: {0}")]
    InsertDb(#[from(diesel::result::Error)] String),
    #[error("failed to insert into db: {0}")]
    GetPgSqlPoolFailed(String),
    /*     #[error("failed to serialize json: {0}")]
    SerdeJson(String), */
    #[error("process data error: {0}")]
    DataProcessError(String),
}

#[derive(Clone)]
pub struct BlockHandler {
    db_client: PgSqlPool,
    block_rx: flume::Receiver<Blocks>,
    shutdown: broadcast::Sender<()>,
}

impl Drop for BlockHandler {
    fn drop(&mut self) {
        info!("BlockHandler drop");
    }
}

impl From<diesel::result::Error> for BlockHandlerError {
    fn from(e: diesel::result::Error) -> Self {
        BlockHandlerError::InsertDb(e.to_string())
    }
}

impl BlockHandler {
    pub fn new(
        db_client: PgSqlPool,
        block_rx: flume::Receiver<Blocks>,
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
            .map_err(|e| BlockHandlerError::GetPgSqlPoolFailed(e.to_string()))?;

        let (block, coinbase, transactions, contracts, calls) = process(header, bodies)
            .await
            .map_err(|e| BlockHandlerError::DataProcessError(e.to_string()))?;

        conn.build_transaction()
            .read_write()
            .serializable()
            .deferrable()
            .run(|conn| {
                batch_insert_block(conn, &vec![block])
                    .map_err(|e| BlockHandlerError::InsertHeaderDb(e.to_string()))?;
                if let Some(c) = coinbase {
                    batch_insert_coinbase(conn, &vec![c])
                        .map_err(|e| BlockHandlerError::InsertHeaderDb(e.to_string()))?;
                }
                batch_insert_transactions(conn, &transactions)
                    .map_err(|e| BlockHandlerError::InsertTransactionDb(e.to_string()))?;

                batch_insert_contracts(conn, &contracts)
                    .map_err(|e| BlockHandlerError::InsertContract(e.to_string()))?;

                batch_insert_calls(conn, &calls)
                    .map_err(|e| BlockHandlerError::InsertCalls(e.to_string()))?;
                Ok(())
            })
    }

    pub async fn start(&mut self) -> Result<(), BlockHandlerError> {
        let mut shutdown = self.shutdown.subscribe();

        loop {
            select! {
                Ok(blocks) = self.block_rx.recv_async() => {
                    for (header, transactions) in blocks {
                        while let Err(e) = self.insert_header_and_txs(&header,&transactions).await {
                            info!("insert_header_and_tx failed {}, retrying",e.to_string());
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

pub fn add_0x_prefix(mut addr: String) -> String {
    if !addr.starts_with("0x") {
        addr.insert_str(0, "0x");
    }

    addr
}
