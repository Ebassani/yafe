use crate::directory::{read_directory};

mod directory;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    let dir = read_directory(".");
    match dir {
        Ok(some_dir) => { some_dir.iter().for_each(|file| println!("{}", file.to_str().unwrap()))}
        Err(error) => { println!("{}", error.message)}
    }
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
