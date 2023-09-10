#![allow(implied_bounds_entailment)]
use std::time::SystemTime;

use anyhow::Result;

use diesel::{
    insert_into, upsert::excluded, ExpressionMethods, Insertable, PgConnection, RunQueryDsl,
};
use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};

use crate::schema::assets;

#[derive(DbEnum, Debug, Clone, Copy, Deserialize, Serialize, PartialEq)]
#[ExistingTypePath = "crate::schema::sql_types::AssetStatus"]
#[serde(rename_all = "snake_case")]
pub enum AssetStatus {
    Alive,
    Delete,
}

#[derive(Insertable, Debug, Clone, Serialize, Deserialize)]
#[diesel(table_name = assets)]
pub struct Assets {
    pub assets_id: String,
    pub assets_utxo_id: String,
    pub assets_owner: String,
    pub amount: i64,
    pub block_height: i64,
    pub create_height: i64,
    pub delete_tx_hash: String,
    pub create_tx_hash: String,
    pub first_seen: SystemTime,
    pub last_seen: SystemTime,
    pub asset_status: AssetStatus,
}

impl Default for Assets {
    fn default() -> Self {
        Assets {
            assets_id: "".to_string(),
            assets_utxo_id: "".to_string(),
            assets_owner: "".to_string(),
            amount: 0,
            block_height: 0,
            create_height: 0,
            delete_tx_hash: "".to_string(),
            create_tx_hash: "".to_string(),
            first_seen: SystemTime::now(),
            last_seen: SystemTime::now(),
            asset_status: AssetStatus::Alive,
        }
    }
}

pub fn batch_insert_assets(connection: &mut PgConnection, records: &Vec<Assets>) -> Result<usize> {
    insert_into(assets::table)
        .values(records)
        .on_conflict(assets::assets_utxo_id)
        .do_update()
        .set((
            assets::amount.eq(excluded(assets::amount)),
            assets::block_height.eq(excluded(assets::block_height)),
            assets::delete_tx_hash.eq(excluded(assets::delete_tx_hash)),
            assets::last_seen.eq(excluded(assets::last_seen)),
            assets::asset_status.eq(excluded(assets::asset_status)),
        ))
        .execute(connection)
        .map_err(|e| anyhow::anyhow!(e.to_string()))
}
