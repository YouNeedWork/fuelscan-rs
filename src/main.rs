use std::str::FromStr;

use fuel_core_client::client::FuelClient;

#[tokio::main]
async fn main() {
    let client =
        FuelClient::from_str("https://beta-3.fuel.network").expect("failed to create client");

    let block = client
        .block_by_height(1418542)
        .await
        .expect("failed to get chain info")
        .unwrap();

    println!("{:?}", block.header.height);
    for tx in block.transactions {
        let full_tx = client
            .transaction(&tx.id.to_string())
            .await
            .expect("failed to get tx")
            .unwrap();

        dbg!(full_tx);
        //println!("{:?}", full_tx);
    }

    println!("Hello, world!");
}
