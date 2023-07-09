use std::collections::HashMap;
use thiserror::Error;

use fuel_core_client::client::{
    schema::{block::Header, schema::Transaction},
    types::TransactionResponse,
};
use rusoto_dynamodb::{AttributeValue, DynamoDb, DynamoDbClient, PutItemInput};
use tokio::{
    select,
    sync::{broadcast, mpsc, oneshot::error},
};
use tracing::info;

use crate::block_read::BlockMsg;

pub struct BlockHandler {
    db_client: DynamoDbClient,
    block_rx: mpsc::Receiver<BlockMsg>,
    shutdown: broadcast::Receiver<()>,
}

#[derive(Debug, Error)]
pub enum BlockHandlerError {
    #[error("failed to insert header into db: {0}")]
    InsertDbError(String),
}

impl BlockHandler {
    pub fn new(
        db_client: DynamoDbClient,
        block_rx: mpsc::Receiver<BlockMsg>,
        shutdown: broadcast::Receiver<()>,
    ) -> Self {
        Self {
            db_client,
            block_rx,
            shutdown,
        }
    }

    async fn insert_header(&mut self, header: Header) -> Result<(), BlockHandlerError> {
        let mut input = PutItemInput::default();
        input.table_name = "headers".to_string();

        let mut item: HashMap<String, AttributeValue> = HashMap::new();
        let hash = header.id.to_string();
        let id = AttributeValue {
            s: Some(hash),
            ..Default::default()
        };
        let height = AttributeValue {
            n: Some(format!("{}", header.height.0)),
            ..Default::default()
        };

        item.insert("id".into(), id);
        item.insert("height".into(), height);

        input.item = item;
        let _ = self
            .db_client
            .put_item(input)
            .await
            .map_err(|e| BlockHandlerError::InsertDbError(e.to_string()))?;

        Ok(())
    }

    async fn insert_tx(&mut self, _tx: Vec<TransactionResponse>) -> Result<(), BlockHandlerError> {
        Ok(())
    }

    pub async fn start(&mut self) -> Result<(), BlockHandlerError> {
        loop {
            select! {
                Some(blocks) = self.block_rx.recv() => {
                    for (header, transactions) in blocks {
                        self.insert_header(header).await?;
                        self.insert_tx(transactions).await?;
                        //info!("received block msg");
                    }
                }
                _ = self.shutdown.recv() => {
                    info!("shutdown signal received");
                    return Ok(());
                }
            }
        }
    }
}
