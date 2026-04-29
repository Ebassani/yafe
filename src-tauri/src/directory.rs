use std::fs::{read_dir};
use std::path::PathBuf;

pub(crate) fn read_directory(path: &str) -> Result<Vec<PathBuf>, FileError> {
    let dir = read_dir(path).map_err(|message| FileError {message: message.to_string()})?;
    
    Ok(dir.filter_map(|dir_item| dir_item.ok().map(|file| file.path())).collect())
}

pub(crate) struct FileError {
    pub(crate) message: String
}
