#![allow(implied_bounds_entailment)]
use anyhow::Result;

use diesel::{insert_into, Insertable, PgConnection, RunQueryDsl};

use serde::{Deserialize, Serialize};

use crate::schema::contracts;

#[derive(Insertable, Debug, Clone, Serialize, Deserialize)]
#[diesel(table_name = contracts)]
pub struct Contract {
    pub contract_id: String,
    pub transaction_id: String,
    pub sender: String,
    pub bytecode: String,
    pub bytecoin_length: i64,
    pub storage_slots: Option<serde_json::Value>,
    pub timestamp: i64,
}

pub fn batch_insert_contracts(
    connection: &mut PgConnection,
    records: &Vec<Contract>,
) -> Result<usize> {
    insert_into(contracts::table)
        .values(records)
        .on_conflict(contracts::contract_id)
        .do_nothing()
        .execute(connection)
        .map_err(|e| anyhow::anyhow!(e.to_string()))
}
