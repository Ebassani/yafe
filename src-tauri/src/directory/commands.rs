use crate::directory::{list, read, FileError, FileInfo};

#[tauri::command]
pub async fn list_directory(path: String) -> Result<Vec<FileInfo>, FileError> {
    tokio::task::spawn_blocking(move || list(&path))
        .await
        .map_err(|err| FileError::DirError(err.to_string()))
        .map_err(|err| FileError::DirError(err.to_string()))?
}

#[tauri::command]
pub async fn read_file(path: String) -> Result<FileInfo, FileError> {
    tokio::task::spawn_blocking(move || read(&path))
        .await
        .map_err(|err| FileError::MetadataError(err.to_string()))
        .map_err(|err| FileError::MetadataError(err.to_string()))?
}