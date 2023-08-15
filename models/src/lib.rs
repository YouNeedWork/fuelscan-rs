use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};

pub mod db;
pub mod schema;

pub type PgSql = ConnectionManager<PgConnection>;
pub type PgSqlPool = Pool<PgSql>;
