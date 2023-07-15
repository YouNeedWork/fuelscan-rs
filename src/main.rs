use std::{str::FromStr, sync::Arc};

use block_read::BlockReader;
use fuel_core_client::client::FuelClient;

use rusoto_core::Region;
use rusoto_dynamodb::DynamoDbClient;

mod block_handle;
mod block_read;
mod database;

#[cfg(not(target_env = "msvc"))]
use tikv_jemallocator::Jemalloc;

use tracing::info;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let db = Arc::new(database::Database::new());

    let (shutdown_tx, _) = tokio::sync::broadcast::channel(1);
    let db_client = DynamoDbClient::new(Region::UsWest1);
    let (block_handler_tx, block_handler_rx) = tokio::sync::mpsc::channel(1000);

    {
        let _db = db.clone();
    }

    let client =
        FuelClient::from_str("https://beta-3.fuel.network").expect("failed to create client");

    let mut block_read = BlockReader::new(
        50,
        client,
        db_client.clone(),
        block_handler_tx,
        shutdown_tx.subscribe(),
    );

    tokio::spawn(async move {
        match block_read.start().await {
            Ok(_) => {}
            Err(e) => {
                panic!("{}", e);
            }
        }
    });

    let mut block_handle = block_handle::BlockHandler::new(
        db_client,
        block_handler_rx,
        shutdown_tx.subscribe(),
        db.clone(),
    );

    tokio::spawn(async move {
        match block_handle.start().await {
            Ok(_) => {}
            Err(e) => {
                panic!("{}", e);
            }
        }
    });

    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
    info!("shutdown signal received");
    shutdown_tx.send(()).unwrap();
}
