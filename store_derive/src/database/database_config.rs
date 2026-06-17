use rusqlite::{params_from_iter, Connection};
use std::path::{PathBuf};
use std::sync::{Arc, Mutex};
use crate::database::store_error::StoreError;
use crate::database::store_query::StoreQuery;
use crate::database::store_result::StoreResult;

pub trait StoreConnector: Send + Sync {
    fn execute(&self, query: StoreQuery) -> StoreResult<usize>;
    fn execute_batch(&self, init_query: &str) -> StoreResult<()>;
}

pub struct SqliteStore {
    db_path: PathBuf,
    conn: Arc<Mutex<Connection>>
}

impl SqliteStore {
    pub fn new(db_path: impl Into<PathBuf>) -> StoreResult<Self> {
        let db_path= db_path.into();
        let conn = Self::create_connection(&db_path)?;

        Ok(Self {
            db_path,
            conn: Arc::new(Mutex::new(conn))
        })
    }

    pub fn with_connection<T>(&self, closure: impl FnOnce(&Connection) -> StoreResult<T>) -> StoreResult<T> {
        let conn = self.conn.lock().map_err(|_| StoreError::Thread(String::from("Mutex poisoned")))?;

        closure(&conn)
    }

    fn create_connection(db_path: &PathBuf) -> StoreResult<Connection> {
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        Ok(Connection::open(db_path)?)
    }
}

impl StoreConnector for SqliteStore {
    fn execute(&self, query: StoreQuery) -> StoreResult<usize> {
        self.with_connection(|conn| {
            let changed = conn.execute(&query.query, params_from_iter(&query.args))?;

            Ok(changed)
        })
    }

    fn execute_batch(&self, batch_query: &str) -> StoreResult<()> {
        self.with_connection(|conn| {
            conn.execute_batch(batch_query)?;

            Ok(())
        })
    }
}