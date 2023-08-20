use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
pub mod block;
pub mod coinbase;
pub mod contract;
pub mod schema;
pub mod transaction;

pub type PgSql = ConnectionManager<PgConnection>;
pub type PgSqlPool = Pool<PgSql>;
