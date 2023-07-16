#[macro_use]
extern crate lazy_static;

use std::{str::FromStr, sync::Arc};

use block_read::BlockReader;
use fuel_core_client::client::FuelClient;

use rusoto_core::{credential::StaticProvider, Region};
use rusoto_dynamodb::DynamoDbClient;

mod block_handle;
mod block_read;
mod database;

#[cfg(not(target_env = "msvc"))]
use tikv_jemallocator::Jemalloc;

use tracing::info;
use tracing_subscriber::FmtSubscriber;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

const REGION: Region = Region::UsWest1;

lazy_static! {
    static ref KEY: String =
        std::env::var("AWS_ACCESS_KEY_ID").expect("AWS_ACCESS_KEY_ID must be set");
}

lazy_static! {
    static ref SECTRYKEY: String =
        std::env::var("AWS_SECRET_ACCESS_KEY").expect("AWS_SECRET_ACCESS_KEY must be set");
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let subscriber = FmtSubscriber::builder()
        .with_ansi(true)
        .with_level(true)
        .with_target(true)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let db = Arc::new(database::Database::new());

    let (shutdown_tx, _) = tokio::sync::broadcast::channel(1);

    let credentials = rusoto_core::credential::AwsCredentials::new(
        KEY.to_string(),
        SECTRYKEY.to_string(),
        None,
        None,
    );

    let credentials_provider = StaticProvider::from(credentials);
    let db_client = DynamoDbClient::new_with(
        rusoto_core::request::HttpClient::new().unwrap(),
        credentials_provider,
        REGION,
    );

    let (block_handler_tx, block_handler_rx) = tokio::sync::mpsc::channel(1000);

    let client =
        FuelClient::from_str("https://beta-3.fuel.network").expect("failed to create client");

    let mut block_read = BlockReader::new(50, client, db_client.clone(), block_handler_tx);

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
