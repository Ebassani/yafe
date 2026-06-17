#[derive(Debug, Clone)]
pub struct StoreQuery {
    pub query: String,
    pub args: Vec<StoreValue>
}

#[derive(Debug, Clone)]
pub enum StoreValue {
    Null,
    Integer(i64),
    Real(f64),
    Text(String),
    Bool(bool),
    Bytes(Vec<u8>),
}

impl rusqlite::types::ToSql for StoreValue {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        use rusqlite::types::{ToSqlOutput, Value};

        match self {
            StoreValue::Null => Ok(ToSqlOutput::Owned(Value::Null)),
            StoreValue::Integer(value) => Ok(ToSqlOutput::Owned(Value::Integer(*value))),
            StoreValue::Real(value) => Ok(ToSqlOutput::Owned(Value::Real(*value))),
            StoreValue::Text(value) => Ok(ToSqlOutput::Owned(Value::Text(value.clone()))),
            StoreValue::Bool(value) => Ok(ToSqlOutput::Owned(Value::Integer(if *value { 1 } else { 0 }))),
            StoreValue::Bytes(value) => Ok(ToSqlOutput::Owned(Value::Blob(value.clone()))),
        }
    }
}