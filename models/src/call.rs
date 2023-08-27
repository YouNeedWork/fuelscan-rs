#![allow(implied_bounds_entailment)]
use anyhow::Result;

use diesel::{insert_into, Insertable, PgConnection, RunQueryDsl};

use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};

use crate::schema::calls;

#[derive(DbEnum, Debug, Clone, Copy, Deserialize, Serialize, PartialEq)]
#[ExistingTypePath = "crate::schema::sql_types::CallType"]
#[serde(rename_all = "snake_case")]
pub enum CallType {
    Contract,
    Transaction,
}

#[derive(Insertable, Debug, Clone, Serialize, Deserialize)]
#[diesel(table_name = calls)]
pub struct Call {
    pub transaction_id: String,
    pub height: i64,
    pub da_height: i64,
    pub block_hash: String,
    pub call_type: CallType,
    pub gas_limit: i64,
    pub gas_price: i64,
    pub gas_used: i64,
    pub sender: String,
    pub receiver: String,
    pub amount: Option<i64>,
    pub asset_id: Option<String>,
    pub payload: Option<String>,
    pub payload_data: Option<String>,
    pub timestamp: i64,
}

pub fn batch_insert_calls(connection: &mut PgConnection, records: &Vec<Call>) -> Result<usize> {
    insert_into(calls::table)
        .values(records)
        .on_conflict(calls::transaction_id)
        .do_nothing()
        .execute(connection)
        .map_err(|e| anyhow::anyhow!(e.to_string()))
}
