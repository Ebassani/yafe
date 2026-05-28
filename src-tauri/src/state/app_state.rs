use std::sync::Arc;
use crate::search::Indexer;

pub(crate) struct AppState {
    pub(crate) indexer: Arc<Indexer>
}