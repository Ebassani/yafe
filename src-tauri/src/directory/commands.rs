use crate::directory::error::FileError;
use crate::directory::directory::list;
use crate::directory::file_info::FileInfo;

#[tauri::command]
pub async fn list_directory(path: String) -> Result<Vec<FileInfo>, FileError> {
    tokio::task::spawn_blocking(move || list(&path))
        .await
        .map_err(|err| FileError::DirError(err.to_string()))
        .map_err(|err| FileError::DirError(err.to_string()))?
}