use std::sync::Arc;
use crate::search::HashIndexer;

pub(crate) struct AppState {
    pub(crate) hash_indexer: Arc<HashIndexer>
}