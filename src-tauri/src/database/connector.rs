use std::fmt::format;
use std::path::{Path, PathBuf};
use directories::BaseDirs;
use rusqlite::{Connection, Result};

pub(crate) struct SqliteConnector {
    pub(crate) db_path: PathBuf
}

impl SqliteConnector {
    fn connect(&self) -> Result<Connection> {
        Connection::open(&self.db_path)
    }

    fn init(&self, init_str: &str) -> Result<()> {
        let conn = self.connect()?;
        conn.execute_batch(init_str)?;
        Ok(())
    }

    pub(crate) fn new(db_name: &str) -> Result<Self, String> {
        let base_dirs = BaseDirs::new()
            .ok_or_else(|| "Could not find user home directory".to_string())?;

        let dir = base_dirs.home_dir().join(".yafe");

        std::fs::create_dir_all(&dir).ok().ok_or_else(|| "Could not create dir")?;

        Ok(Self { db_path: dir.join(format!("{}.sqlite", db_name))})
    }
}