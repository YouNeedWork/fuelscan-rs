#![allow(implied_bounds_entailment)]
use std::time::SystemTime;

use anyhow::Result;

use diesel::{insert_into, Insertable, PgConnection, RunQueryDsl};
use serde::{Deserialize, Serialize};

use crate::schema::assets;

#[derive(Insertable, Debug, Clone, Serialize, Deserialize)]
#[diesel(table_name = assets)]
pub struct Assets {
    pub assets_id: String,
    pub assets_hash: String,
    pub amount: i64,
    pub block_height: i64,
    pub create_height: i64,
    pub create_tx_hash: String,
    pub first_seen: SystemTime,
    pub last_seen: SystemTime,
}

impl Default for Assets {
    fn default() -> Self {
        Assets {
            assets_id: "".to_string(),
            assets_hash: "".to_string(),
            amount: 0,
            block_height: 0,
            create_height: 0,
            create_tx_hash: "".to_string(),
            first_seen: SystemTime::now(),
            last_seen: SystemTime::now(),
        }
    }
}
