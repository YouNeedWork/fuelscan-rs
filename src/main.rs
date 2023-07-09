use std::str::FromStr;

use fuel_core_client::client::FuelClient;
use futures::future;
use rusoto_core::Region;
use rusoto_dynamodb::{DynamoDb, DynamoDbClient, ListTablesInput};

mod block_handle;
mod block_read;

use block_read::BlockReader;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let (shutdown_tx, _) = tokio::sync::broadcast::channel(1);
    let db_client = DynamoDbClient::new(Region::UsWest1);
    let (block_handler_tx, block_handler_rx) = tokio::sync::mpsc::channel(1000);

    let client =
        FuelClient::from_str("https://beta-3.fuel.network").expect("failed to create client");

    let mut block_read = BlockReader::new(
        50,
        client,
        db_client.clone(),
        shutdown_tx.subscribe(),
        block_handler_tx,
    );

    tokio::spawn(async move {
        match block_read.start().await {
            Ok(_) => {}
            Err(e) => {
                panic!("{}", e);
            }
        }
    });

    let mut block_handle =
        block_handle::BlockHandler::new(db_client, block_handler_rx, shutdown_tx.subscribe());

    tokio::spawn(async move {
        match block_handle.start().await {
            Ok(_) => {}
            Err(e) => {
                panic!("{}", e);
            }
        }
    });

    future::pending::<()>().await;
    shutdown_tx.send(()).unwrap();
}
