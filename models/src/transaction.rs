use anyhow::Result;

use diesel::{insert_into, Insertable, PgConnection, RunQueryDsl};
use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};

use crate::schema::transactions;

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
#[diesel(table_name = transactions)]
pub struct Transaction {
    pub id: String,
    pub height: i64,
    pub da_height: i64,
    pub block_hash: String,
    pub tx_type: Option<TxType>,
    pub gas_limit: i64,
    pub gas_price: i64,
    pub gas_used: i64,
    pub timestamp: i64,
    pub sender: Option<String>,
    pub status: TxStatus,
    pub reason: String,
    pub input: Option<serde_json::Value>,
    pub output: Option<serde_json::Value>,
    pub receipts: Option<serde_json::Value>,
}

pub fn batch_insert_transactions(
    connection: &mut PgConnection,
    records: &Vec<Transaction>,
) -> Result<usize> {
    insert_into(transactions::table)
        .values(records)
        .on_conflict(transactions::id)
        .do_nothing()
        .execute(connection)
        .map_err(|e| anyhow::anyhow!(e.to_string()))
}
