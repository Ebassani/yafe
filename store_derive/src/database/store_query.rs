#[derive(Debug, Clone)]
pub struct StoreQuery {
    query: String,
    args: Vec<StoreValue>
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