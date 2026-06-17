use crate::database::store_error::StoreError;

pub type StoreResult<T> = Result<T, StoreError>;