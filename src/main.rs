use std::str::FromStr;

use fuel_core_client::client::FuelClient;

mod block_read;
use block_read::BlockReader;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let (shutdown_tx, _) = tokio::sync::broadcast::channel(1);

    let client =
        FuelClient::from_str("https://beta-3.fuel.network").expect("failed to create client");

    let mut block_read = BlockReader::new(client, shutdown_tx.subscribe());
    tokio::spawn(async move { block_read.start().await });

    tokio::signal::ctrl_c().await.unwrap();
    println!("🎩 Ctrl-C received, shutting down");
    shutdown_tx.send(()).unwrap();
}
