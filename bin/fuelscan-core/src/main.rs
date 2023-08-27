use block_read::BlockReader;

use diesel::{r2d2::ConnectionManager, PgConnection};
use flume::unbounded;
use fuel_core_client::client::FuelClient;
use models::block::get_last_block_height;
use std::str::FromStr;

mod block_handle;
mod block_read;

use tracing::info;
use tracing_subscriber::FmtSubscriber;

use crate::block_read::FetchBlockResult;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let subscriber = FmtSubscriber::builder()
        .with_ansi(true)
        .with_level(true)
        .with_line_number(true)
        //.with_file(true)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let manager = ConnectionManager::<PgConnection>::new(std::env::var("DATABASE_URL").unwrap());
    let pool: diesel::r2d2::Pool<ConnectionManager<PgConnection>> = diesel::r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

    let (shutdown_tx, _) = tokio::sync::broadcast::channel(1);

    let (block_handler_tx, block_handler_rx) = unbounded::<Vec<FetchBlockResult>>();

    let client =
        FuelClient::from_str("https://beta-3.fuel.network").expect("failed to create client");

    let mut block_read = BlockReader::new(20, client, block_handler_tx);
    let height = get_last_block_height(&mut pool.get().unwrap()) as u64;

    tokio::spawn(async move {
        match block_read.start(height).await {
            Ok(_) => {}
            Err(e) => {
                panic!("{}", e);
            }
        }
    });

    let block_handle = block_handle::BlockHandler::new(pool, block_handler_rx, shutdown_tx.clone());

    for _ in 0..10 {
        let mut block_handle = block_handle.clone();
        tokio::spawn(async move {
            match block_handle.start().await {
                Ok(_) => {}
                Err(e) => {
                    panic!("{}", e);
                }
            }
        });
    }

    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
    shutdown_tx.send(()).unwrap();
}
