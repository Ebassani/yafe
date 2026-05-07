use crate::directory::errors::FileError;
use crate::directory::read::read_directory;
use crate::directory::types::FileInfo;

#[tauri::command]
pub async fn read_dir(path: String) -> Result<Vec<FileInfo>, FileError> {
    tokio::task::spawn_blocking(move || read_directory(&path))
        .await
        .map_err(|err| FileError::DirError(err.to_string()))
        .map_err(|err| FileError::DirError(err.to_string()))?
}