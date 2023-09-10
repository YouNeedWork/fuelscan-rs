#![allow(implied_bounds_entailment)]

use anyhow::Result;
use std::time::SystemTime;

use diesel::{
    insert_into, upsert::excluded, ExpressionMethods, Insertable, PgConnection, RunQueryDsl,
};
use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};

use crate::schema::accounts;

#[derive(DbEnum, Debug, Clone, Copy, Deserialize, Serialize, PartialEq)]
#[ExistingTypePath = "crate::schema::sql_types::AccountType"]
#[serde(rename_all = "snake_case")]

pub enum AccountType {
    Account,
    Contract,
}

#[derive(Insertable, Debug, Clone, Serialize, Deserialize, Builder)]
#[diesel(table_name = accounts)]
#[builder(setter(into))]
pub struct Account {
    pub account_hash: String,
    pub account_code: Option<String>,
    pub account_name: Option<String>,
    pub account_type: AccountType,
    pub verified: bool,
    pub gas_used: i64,
    pub transactions_count: i64,
    pub token_transfers_count: i64,
    pub sender_count: i64,
    pub recever_count: i64,
    pub decompiled: bool,
    pub inserted_at: SystemTime,
    pub updated_at: SystemTime,
}

impl Default for Account {
    fn default() -> Self {
        Self {
            account_hash: Default::default(),
            account_code: Default::default(),
            account_name: Default::default(),
            account_type: AccountType::Account,
            verified: Default::default(),
            gas_used: Default::default(),
            transactions_count: Default::default(),
            token_transfers_count: Default::default(),
            sender_count: Default::default(),
            recever_count: Default::default(),
            decompiled: Default::default(),
            inserted_at: SystemTime::now(),
            updated_at: SystemTime::now(),
        }
    }
}

pub fn batch_insert_assets(connection: &mut PgConnection, records: &Vec<Account>) -> Result<usize> {
    insert_into(accounts::table)
        .values(records)
        .on_conflict(accounts::account_hash)
        .do_update()
        .set((
            accounts::account_name.eq(excluded(accounts::account_name)),
            accounts::verified.eq(excluded(accounts::verified)),
            accounts::gas_used.eq(excluded(accounts::gas_used)),
            accounts::transactions_count.eq(excluded(accounts::transactions_count)),
            accounts::sender_count.eq(excluded(accounts::sender_count)),
            accounts::recever_count.eq(excluded(accounts::recever_count)),
            accounts::updated_at.eq(excluded(accounts::updated_at)),
        ))
        .execute(connection)
        .map_err(|e| anyhow::anyhow!(e.to_string()))
}
