#[allow(implied_bounds_entailment)]
use anyhow::Result;

use diesel::{insert_into, Insertable, PgConnection, RunQueryDsl};
use fuel_core_client::client::schema::block::Header;
use serde::{Deserialize, Serialize};

use crate::schema::blocks;

#[derive(Insertable, Debug, Clone, Serialize, Deserialize)]
#[diesel(table_name = blocks)]
pub struct Block {
    pub id: String,
    pub height: i32,
    pub da_height: i32,
    pub application_hash: String,
    pub output_messages_root_hash: String,
    pub transactions_root: String,
    pub prev_root: String,
    pub coinbase: Option<String>,
    pub coinbase_hash: Option<String>,
    pub coinbase_amount: Option<String>,
    pub count: i32,
    pub timestamp: i32,
}

/*
impl From<Header> for Block {
    fn from(header: Header) -> Self {
        Self {
            id: header.id.to_string(),
            height: header.height,
            da_height: header.da_height,
            application_hash: header.application_hash,
            output_messages_root_hash: header.output_messages_root_hash,
            transactions_root: header.transactions_root,
            prev_root: header.prev_root,
            coinbase: header.coinbase,
            coinbase_hash: header.coinbase_hash,
            coinbase_amount: header.coinbase_amount,
            count: header.count,
            timestamp: header.timestamp,
        }
}
} */

pub fn batch_insert_block(connection: &mut PgConnection, records: &Vec<Block>) -> Result<usize> {
    insert_into(blocks::table)
        .values(records)
        .execute(connection)
        .map_err(|e| anyhow::anyhow!(e.to_string()))
}
