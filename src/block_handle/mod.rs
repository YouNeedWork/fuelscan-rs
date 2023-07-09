use fuel_core_types::fuel_tx::field::*;

use std::collections::HashMap;
use thiserror::Error;

use fuel_core_client::client::{
    schema::{block::Header, schema::__fields::ContractBalanceConnection},
    types::TransactionResponse,
};
use rusoto_dynamodb::{
    AttributeValue, AttributeValueUpdate, DynamoDb, DynamoDbClient, PutItemInput, UpdateItemInput,
};
use tokio::{
    select,
    sync::{broadcast, mpsc},
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
    InsertHeaderDbError(String),
    #[error("failed to insert transaction into db: {0}")]
    InsertTransactionDbError(String),
    #[error("failed to update check_point: {0}")]
    InsertUpdateCheckPointError(String),
    #[error("failed to insert into db: {0}")]
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

    async fn insert_header(&mut self, header: &Header) -> Result<(), BlockHandlerError> {
        let mut input = PutItemInput::default();
        input.table_name = "fuel_headers".to_string();

        let mut item: HashMap<String, AttributeValue> = HashMap::new();
        let hash = header.id.to_string();
        let id = AttributeValue {
            s: Some(hash),
            ..Default::default()
        };
        item.insert("id".into(), id);

        let height = AttributeValue {
            n: Some(format!("{}", header.height.0)),
            ..Default::default()
        };
        item.insert("height".into(), height);

        let da_height = AttributeValue {
            n: Some(format!("{}", header.da_height.0)),
            ..Default::default()
        };
        item.insert("da_height".into(), da_height);

        let timestamp = AttributeValue {
            n: Some(format!("{}", header.time.0.to_unix())),
            ..Default::default()
        };
        item.insert("timestamp".into(), timestamp);

        let prev_root = AttributeValue {
            s: Some(format!("{}", header.prev_root)),
            ..Default::default()
        };
        item.insert("prev_root".into(), prev_root);
        let transactions_root = AttributeValue {
            s: Some(format!("{}", header.transactions_root)),
            ..Default::default()
        };
        item.insert("transactions_root".into(), transactions_root);
        let output_messages_root = AttributeValue {
            s: Some(format!("{}", header.output_messages_root)),
            ..Default::default()
        };
        item.insert("output_messages_root".into(), output_messages_root);
        let application_hash = AttributeValue {
            s: Some(format!("{}", header.application_hash)),
            ..Default::default()
        };
        item.insert("application_hash".into(), application_hash);
        let transactions_count = AttributeValue {
            n: Some(format!("{}", header.transactions_count.0)),
            ..Default::default()
        };
        item.insert("transactions_count".into(), transactions_count);
        let output_messages_count = AttributeValue {
            n: Some(format!("{}", header.output_messages_count.0)),
            ..Default::default()
        };
        item.insert("output_messages_count".into(), output_messages_count);

        input.item = item;
        let _ = self
            .db_client
            .put_item(input)
            .await
            .map_err(|e| BlockHandlerError::InsertHeaderDbError(e.to_string()))?;

        Ok(())
    }

    async fn insert_tx(
        client: &mut DynamoDbClient,
        header: &Header,
        (hash, tx): (String, TransactionResponse),
    ) -> Result<(), BlockHandlerError> {
        let mut input = PutItemInput::default();
        input.table_name = "fuel_transactions".to_string();

        let mut item: HashMap<String, AttributeValue> = HashMap::new();

        let id = AttributeValue {
            s: Some(hash),
            ..Default::default()
        };
        item.insert("id".into(), id);

        let tx_json = tx.transaction.to_json();
        let transaction: AttributeValue = AttributeValue {
            s: Some(tx_json),
            ..Default::default()
        };
        item.insert("transaction".into(), transaction);

        let block_hash: AttributeValue = AttributeValue {
            s: Some(header.id.to_string()),
            ..Default::default()
        };
        item.insert("block_hash".into(), block_hash);

        let height = AttributeValue {
            n: Some(format!("{}", header.height.0)),
            ..Default::default()
        };
        item.insert("height".into(), height);

        let da_height = AttributeValue {
            n: Some(format!("{}", header.da_height.0)),
            ..Default::default()
        };
        item.insert("da_height".into(), da_height);

        let status = serde_json::to_string(&tx.status).map_err(|e| {
            BlockHandlerError::InsertDbError(format!("failed to serialize status: {}", e))
        })?;

        let status: AttributeValue = AttributeValue {
            s: Some(status),
            ..Default::default()
        };
        item.insert("status".into(), status);

        if tx.transaction.is_create() {
            //contract deplyment or somethings
            let create = tx.transaction.as_create().unwrap();
            let transaction_type = AttributeValue {
                s: Some("create".into()),
                ..Default::default()
            };
            item.insert("transaction_type".into(), transaction_type);

            let gas_price = AttributeValue {
                n: Some(format!("{}", create.gas_price())),
                ..Default::default()
            };
            item.insert("gas_price".into(), gas_price);
            let gas_limit = AttributeValue {
                n: Some(format!("{}", create.gas_limit())),
                ..Default::default()
            };
            item.insert("gas_limit".into(), gas_limit);

        /*             create.inputs();
        create.outputs(); */
        } else if tx.transaction.is_mint() {
            //system Msg?
            let mint = tx.transaction.as_mint().unwrap();
            mint.outputs();
            let transaction_type = AttributeValue {
                s: Some("mint".into()),
                ..Default::default()
            };
            item.insert("transaction_type".into(), transaction_type);
            /* mint.();
            mint.gas_limit(); */
        } else if tx.transaction.is_script() {
            //transfer or contract call
            let script = tx.transaction.as_script().unwrap();

            let transaction_type = AttributeValue {
                s: Some("script".into()),
                ..Default::default()
            };
            item.insert("transaction_type".into(), transaction_type);

            let gas_price = AttributeValue {
                n: Some(format!("{}", script.gas_price())),
                ..Default::default()
            };
            item.insert("gas_price".into(), gas_price);
            let gas_limit = AttributeValue {
                n: Some(format!("{}", script.gas_limit())),
                ..Default::default()
            };
            item.insert("gas_limit".into(), gas_limit);
        } else {
            unimplemented!();
        }

        input.item = item;
        let _ = client
            .put_item(input)
            .await
            .map_err(|e| BlockHandlerError::InsertTransactionDbError(e.to_string()))?;

        Ok(())
    }

    async fn update_check_point(&mut self, check_point: u64) -> Result<(), BlockHandlerError> {
        let mut input = PutItemInput::default();
        input.table_name = "fuel_check_point".to_string();

        let mut item: HashMap<String, AttributeValue> = HashMap::new();

        let id = AttributeValue {
            n: Some("1".into()),
            ..Default::default()
        };

        item.insert("id".into(), id);
        let chain = AttributeValue {
            n: Some("1".into()),
            ..Default::default()
        };
        item.insert("chain".into(), chain);

        let check_point = AttributeValue {
            n: Some(format!("{}", check_point)),
            ..Default::default()
        };
        item.insert("check_point".into(), check_point);

        input.item = item;
        let _ = self
            .db_client
            .put_item(input)
            .await
            .map_err(|e| BlockHandlerError::InsertUpdateCheckPointError(e.to_string()))?;

        Ok(())
    }

    pub async fn start(&mut self) -> Result<(), BlockHandlerError> {
        loop {
            select! {
                Some(blocks) = self.block_rx.recv() => {
                    for (header, transactions) in blocks {
                        self.insert_header(&header).await?;

                        for tx in transactions {
                            Self::insert_tx(&mut self.db_client,&header, tx).await?
                        }

                        self.update_check_point(header.height.0).await?;
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
