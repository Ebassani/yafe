use rusqlite::{Connection};
use std::path::{PathBuf};
use std::sync::{Arc, Mutex};
use crate::database::store_query::StoreQuery;
use crate::database::store_result::StoreResult;

trait StoreConnector {
    fn execute(&self, query: StoreQuery) -> StoreResult<usize>;
    fn init(&self, init_query: &str) -> StoreResult<()>;
}

pub struct SqliteStore {
    db_path: PathBuf,
    conn: Arc<Mutex<Connection>>
}

pub(crate) trait Connector<T>: Send + Sync {
    fn connect(&self) -> StoreResult<T>;
}

pub(crate) struct SqliteConnector {
    pub db_path: PathBuf,
}

impl SqliteConnector {
    pub(crate) fn new(db_path: PathBuf) -> StoreResult<Self> {
        std::fs::create_dir_all(&db_path).map_err(|_| rusqlite::Error::InvalidPath(db_path.clone().into()))?;

        Ok(Self { db_path })
    }
}

impl Connector<Connection> for SqliteConnector {
    fn connect(&self) -> StoreResult<Connection> {
        Ok(Connection::open(&self.db_path)?)
    }
}