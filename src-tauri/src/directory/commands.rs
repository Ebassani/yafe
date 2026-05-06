use crate::directory::errors::FileError;
use crate::directory::read::read_directory;
use crate::directory::types::FileInfo;

#[tauri::command]
pub fn read_dir(path: String) -> Result<Vec<FileInfo>, FileError> {
    read_directory(&path)
}