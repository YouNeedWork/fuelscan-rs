#[allow(implied_bounds_entailment)]
use anyhow::Result;

use diesel::{insert_into, Insertable, PgConnection, RunQueryDsl};

use serde::{Deserialize, Serialize};

use crate::schema::coinbases;

#[derive(Insertable, Debug, Clone, Serialize, Deserialize)]
#[diesel(table_name = coinbases)]
pub struct Coinbase {
    pub id: String,
    pub height: i64,
    pub da_height: i64,
    pub block_hash: String,
    pub amount: Option<i64>,
    pub coinbase: Option<String>,
    pub timestamp: Option<i64>,
}

pub fn batch_insert_coinbase(
    connection: &mut PgConnection,
    records: &Vec<Coinbase>,
) -> Result<usize> {
    insert_into(coinbases::table)
        .values(records)
        .on_conflict(coinbases::id)
        .do_nothing()
        .execute(connection)
        .map_err(|e| anyhow::anyhow!(e.to_string()))
}
