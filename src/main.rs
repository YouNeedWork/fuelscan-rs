use std::str::FromStr;

use fuel_core_client::client::FuelClient;
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

    let mut block_read = BlockReader::new(50, client, shutdown_tx.subscribe(), block_handler_tx);
    tokio::spawn(async move { block_read.start().await });

    let mut block_handle =
        block_handle::BlockHandler::new(db_client, block_handler_rx, shutdown_tx.subscribe());
    tokio::spawn(async move { block_handle.start().await });

    /*     let list_tables_input: ListTablesInput = Default::default();

    match client.list_tables(list_tables_input).await {
        Ok(output) => match output.table_names {
            Some(table_name_list) => {
                println!("Tables in database:");

                for table_name in table_name_list {
                    println!("{}", table_name);
                }
            }
            None => println!("No tables in database!"),
        },
        Err(error) => {
            println!("Error: {:?}", error);
        }
    } */

    tokio::signal::ctrl_c().await.unwrap();
    println!("🎩 Ctrl-C received, shutting down");
    shutdown_tx.send(()).unwrap();
}
