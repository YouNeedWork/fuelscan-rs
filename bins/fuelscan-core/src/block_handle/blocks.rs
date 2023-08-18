use anyhow::Result;
use diesel::PgConnection;
use fuel_core_client::client::schema::block::Header;
use models::db::{batch_insert_block, Block};

pub fn init_block_by_with_header(header: &Header) -> Block {
    Block {
        id: header.id.to_string(),
        height: header.height.0 as i64,
        da_height: header.da_height.0 as i64,
        application_hash: header.application_hash.to_string(),
        output_messages_root_hash: header.output_messages_root.to_string(),
        transactions_root: header.transactions_root.to_string(),
        prev_root: header.prev_root.to_string(),
        coinbase: None,
        coinbase_hash: None,
        coinbase_amount: None,
        transaction_count: header.transactions_count.0 as i64,
        output_message_count: header.output_messages_count.0 as i64,
        timestamp: header.time.clone().to_unix(),
    }
}

pub fn insert_header(conn: &mut PgConnection, header: &Header) -> Result<()> {
    let block = Block {
        id: header.id.to_string(),
        height: header.height.0 as i64,
        da_height: header.da_height.0 as i64,
        application_hash: header.application_hash.to_string(),
        output_messages_root_hash: header.output_messages_root.to_string(),
        transactions_root: header.transactions_root.to_string(),
        prev_root: header.prev_root.to_string(),
        coinbase: None,
        coinbase_hash: None,
        coinbase_amount: None,
        transaction_count: header.transactions_count.0 as i64,
        output_message_count: header.output_messages_count.0 as i64,
        timestamp: header.time.clone().to_unix(),
    };

    batch_insert_block(conn, &vec![block])?;

    Ok(())
}
