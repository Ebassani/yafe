use std::{fs, path::{Path, PathBuf}};
use crate::database::store_error::StoreError;
use crate::database::store_result::StoreResult;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct StoreConfig {
    pub database: DatabaseConfig,

    #[serde(default)]
    pub sqlite: SqliteOptions,
}

impl StoreConfig {
    pub fn from_file(path: impl AsRef<Path>) -> StoreResult<Self> {
        let _ = dotenvy::dotenv();

        let raw = fs::read_to_string(path)?;
        let config: StoreConfig = toml::from_str(&raw)
            .map_err(|err| StoreError::Config(err.to_string()))?;

        Ok(config)
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "provider", rename_all = "snake_case")]
pub enum DatabaseConfig {
    Sqlite {
        path: Option<PathBuf>,
        path_env: Option<String>,
    },

    Postgres {
        url: Option<String>,
        url_env: Option<String>,
    },
}

#[derive(Debug, Clone, Deserialize)]
pub struct SqliteOptions {
    #[serde(default = "default_true")]
    pub foreign_keys: bool,

    #[serde(default = "default_journal_mode")]
    pub journal_mode: String,

    #[serde(default = "default_busy_timeout_ms")]
    pub busy_timeout_ms: u64,
}

impl Default for SqliteOptions {
    fn default() -> Self {
        Self {
            foreign_keys: true,
            journal_mode: default_journal_mode(),
            busy_timeout_ms: default_busy_timeout_ms(),
        }
    }
}

fn default_true() -> bool {
    true
}

fn default_journal_mode() -> String {
    "wal".to_string()
}

fn default_busy_timeout_ms() -> u64 {
    5000
}