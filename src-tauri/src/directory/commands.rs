use crate::directory::{list, list_user_dirs, read, FileError, FileInfo, UserDir};

#[tauri::command]
pub async fn list_directory(path: String) -> Result<Vec<FileInfo>, FileError> {
    tokio::task::spawn_blocking(move || list(path))
        .await
        .map_err(|err| FileError::directory(err.to_string()))
        .map_err(|err| FileError::directory(err.to_string()))?
}

#[tauri::command]
pub async fn list_user_directories() -> Result<Vec<UserDir>, FileError> {
    tokio::task::spawn_blocking(list_user_dirs)
        .await
        .map_err(|err| FileError::directory(err.to_string()))
}

#[tauri::command]
pub async fn read_file(path: String) -> Result<FileInfo, FileError> {
    tokio::task::spawn_blocking(move || read(&path))
        .await
        .map_err(|err| FileError::metadata(err.to_string()))
        .map_err(|err| FileError::metadata(err.to_string()))?
}