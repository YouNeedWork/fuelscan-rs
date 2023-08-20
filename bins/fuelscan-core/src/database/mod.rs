#![allow(dead_code)]
use num_enum::IntoPrimitive;
use std::sync::Arc;

use anyhow::Result;
use kvdb_rocksdb::{Database as RKDB, DatabaseConfig};
use tokio::sync::RwLock;

pub type DB = Arc<Database>;

#[derive(IntoPrimitive)]
#[repr(u32)]
pub enum DatabaseName {
    Slot,
    Contract,
    Address,
    Calldata,
    Transaction,
    Script,
    ScriptData,
}

pub struct Database {
    pub db: RwLock<RKDB>,
}

impl Database {
    pub fn new() -> Self {
        let mut cfg = DatabaseConfig::default();
        cfg.columns = 7;
        let db = RKDB::open(&cfg, "./fuelscan/rocksdb").unwrap();

        Self {
            db: RwLock::new(db),
        }
    }

    pub async fn get<T>(&self, col: DatabaseName, key: &[u8]) -> Option<T>
    where
        T: for<'a> serde::Deserialize<'a>,
    {
        let db = self.db.read().await;

        db.get(col.into(), key)
            .unwrap()
            .as_ref()
            .map(|v| flexbuffers::from_slice::<T>(v).ok().unwrap())
    }

    pub async fn set_raw(&self, col: DatabaseName, key: &[u8], v: &[u8]) -> Result<()> {
        let db = self.db.write().await;
        let mut tx = db.transaction();
        tx.put(col.into(), key, v);
        Ok(db.write(tx)?)
    }

    pub async fn set<T>(&self, col: DatabaseName, key: &[u8], v: &T) -> Result<()>
    where
        T: serde::Serialize,
    {
        let mut s = flexbuffers::FlexbufferSerializer::new();
        v.serialize(&mut s).unwrap();

        {
            let db = self.db.write().await;
            let mut tx = db.transaction();
            tx.put(col.into(), key, &s.take_buffer());
            Ok(db.write(tx)?)
        }
    }
}
