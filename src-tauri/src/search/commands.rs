use tauri::State;
use crate::search::{IndexedEntry, Indexer};
use crate::state::AppState;

#[tauri::command]
pub(crate) fn search_index(
    query: String,
    state: State<'_, AppState>,
) -> Vec<IndexedEntry> {
    state.hash_indexer.search_and_get_indexed_entries(&query)
}