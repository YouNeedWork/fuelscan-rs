use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};

#[macro_use]
extern crate derive_builder;

pub mod account;
pub mod assets;
pub mod block;
pub mod call;
pub mod coinbase;
pub mod contract;
pub mod schema;
pub mod transaction;

pub type PgSql = ConnectionManager<PgConnection>;
pub type PgSqlPool = Pool<PgSql>;
