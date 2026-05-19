mod directory;

use crate::directory::{list_directory, list_directory_stream, list_user_directories, read_file};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            list_directory,
            list_directory_stream,
            read_file,
            list_user_directories
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
