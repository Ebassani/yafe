use rusqlite::{Connection};
use std::path::{PathBuf};
use std::sync::{Arc, Mutex};
use crate::database::store_query::StoreQuery;
use crate::database::store_result::StoreResult;

pub trait StoreConnector: Send + Sync {
    fn execute(&self, query: StoreQuery) -> StoreResult<usize>;
    fn init(&self, init_query: &str) -> StoreResult<()>;
}

pub struct SqliteStore {
    db_path: PathBuf,
    conn: Arc<Mutex<Connection>>
}

impl SqliteStore {
    pub fn new(db_path: PathBuf) -> StoreResult<Self> {
        let conn = Self::create_connection(&db_path)?;

        Ok(Self {
            db_path,
            conn: Arc::new(Mutex::new(conn))
        })
    }

    fn create_connection(db_path: &PathBuf) -> StoreResult<Connection> {
        let db_path: PathBuf = db_path.into();

        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        Ok(Connection::open(&db_path)?)
    }
}

impl StoreConnector for SqliteStore {
    fn execute(&self, query: StoreQuery) -> StoreResult<usize> {
        todo!()
    }

    fn init(&self, init_query: &str) -> StoreResult<()> {
        todo!()
    }
}