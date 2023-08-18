#[allow(implied_bounds_entailment)]
use anyhow::Result;

use diesel::{Insertable, PgConnection, RunQueryDsl};
use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};

use crate::schema::transctions;

#[derive(DbEnum, Debug, Clone, Copy, Deserialize, Serialize, PartialEq)]
#[ExistingTypePath = "crate::schema::sql_types::TxStatus"]
#[serde(rename_all = "snake_case")]
pub enum TxStatus {
    Success,
    Failed,
}

#[derive(DbEnum, Debug, Clone, Copy, Deserialize, Serialize, PartialEq)]
#[ExistingTypePath = "crate::schema::sql_types::TxType"]
#[serde(rename_all = "snake_case")]
pub enum TxType {
    Call,
    Deploy,
}

#[derive(Insertable, Debug, Clone, Serialize, Deserialize)]
#[diesel(table_name = transctions)]
pub struct Transaction {
    pub id: String,
    pub height: i64,
    pub da_height: i64,
    pub block_hash: String,
    pub tx_type: Option<TxType>,
    pub gas_limit: Option<String>,
    pub gas_price: Option<String>,
    pub timestamp: i64,
    pub sender: Option<String>,
    pub status: Option<TxStatus>,
    pub reason: Option<String>,
    pub input: Option<serde_json::Value>,
    pub output: Option<serde_json::Value>,
}

/* pub fn batch_insert_block(connection: &mut PgConnection, records: &Vec<Block>) -> Result<usize> {
    insert_into(blocks::table)
        .values(records)
        .execute(connection)
        .map_err(|e| anyhow::anyhow!(e.to_string()))
} */

/* transctions (id) {
    id -> Varchar,
    height -> Int8,
    block_hash -> Varchar,
    tx_type -> Nullable<TransactionType>,
    da_height -> Int4,
    gas_limit -> Varchar,
    gas_price -> Varchar,
    timestamp -> Int4,
    sender -> Nullable<Varchar>,
    status -> Nullable<Status>,
    reason -> Nullable<Varchar>,
    input -> Nullable<Json>,
    output -> Nullable<Json>,
}
 */
