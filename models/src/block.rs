#[allow(implied_bounds_entailment)]
use anyhow::Result;

use diesel::{insert_into, Insertable, PgConnection, QueryDsl, RunQueryDsl};
use serde::{Deserialize, Serialize};

use crate::schema::blocks;

#[derive(Insertable, Debug, Clone, Serialize, Deserialize)]
#[diesel(table_name = blocks)]
pub struct Block {
    pub id: String,
    pub height: i64,
    pub da_height: i64,
    pub application_hash: String,
    pub output_messages_root_hash: String,
    pub transactions_root: String,
    pub prev_root: String,
    pub coinbase: Option<String>,
    pub coinbase_hash: Option<String>,
    pub coinbase_amount: Option<i64>,
    pub transaction_count: i64,
    pub output_message_count: i64,
    pub timestamp: i64,
}

pub fn batch_insert_block(connection: &mut PgConnection, records: &Vec<Block>) -> Result<usize> {
    insert_into(blocks::table)
        .values(records)
        .execute(connection)
        .map_err(|e| anyhow::anyhow!(e.to_string()))
}

pub fn get_last_block_height(connection: &mut PgConnection) -> i64 {
    use crate::schema::blocks::dsl::*;
    use diesel::dsl::max;

    blocks
        .select(max(height))
        .first::<Option<i64>>(connection)
        .unwrap_or(Some(0))
        .unwrap_or_default()
}
