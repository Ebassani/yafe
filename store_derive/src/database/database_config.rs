use rusqlite::{Connection, Result};
use std::path::Path;

pub(crate) trait Connector<T>: Send + Sync {
    fn connect(&self) -> Result<T>;
    fn init(&self, init_str: &str) -> Result<()>;
}

pub(crate) struct SqliteConnector {
    pub db_path: String,
}

impl SqliteConnector {
    pub(crate) fn new(workspace: &str, db_name: &str) -> Result<Self> {
        let base = Path::new(workspace).join(".tyde");
        let db_path = base
            .join(format!("{}.db", db_name))
            .to_string_lossy()
            .to_string();

        std::fs::create_dir_all(&base).map_err(|_| rusqlite::Error::InvalidPath(base.into()))?;

        Ok(Self { db_path })
    }
}

impl Connector<Connection> for SqliteConnector {
    fn connect(&self) -> Result<Connection> {
        Connection::open(&self.db_path)
    }

    fn init(&self, init_str: &str) -> Result<()> {
        let conn = self.connect()?;
        conn.execute_batch(init_str)?;
        Ok(())
    }
}