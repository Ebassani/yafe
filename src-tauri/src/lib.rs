mod directory;
mod search;

mod queue;

mod state;

use std::sync::Arc;
use crate::directory::{list_directory, list_directory_stream, list_user_directories, list_user_dirs, read_file};
use crate::search::{search_index, CrawlCoordinator, Indexer};
use crate::state::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let indexer = Arc::new(Indexer::new());

    tauri::Builder::default()
        .manage(AppState {
            indexer: Arc::clone(&indexer)
        })
        .setup(move |_app| {
            let indexer = Arc::clone(&indexer);

            tauri::async_runtime::spawn(async move {
                let coordinator = Arc::new(CrawlCoordinator::new());

                let mut user_dirs: Vec<String> = Vec::new();

                for dir in list_user_dirs() {
                    user_dirs.push(dir.dir_path);
                }

                Arc::clone(&coordinator)
                    .crawl_and_index(
                        user_dirs,
                        indexer,
                    )
                    .await;
            });

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            list_directory,
            list_directory_stream,
            read_file,
            list_user_directories,
            search_index
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
