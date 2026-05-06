use std::fs::{read_dir, FileType};
use std::time::{SystemTime, UNIX_EPOCH};
use serde::Serialize;
use crate::directory::DirFileType::{Dir, File, Symlink};

pub(crate) fn read_directory(path: &str) -> Result<Vec<FileInfo>, FileError> {
    let dir = read_dir(path).map_err(|message| FileError::DirError(message.to_string()))?;

    let info: Result<Vec<FileInfo>, FileError> = dir.map(|dir_item|  {
        let file = dir_item.map_err(|some_err| FileError::DirError(some_err.to_string()))?;

        let metadata = file.metadata().map_err(|err| FileError::MetadataError(err.to_string()))?;

        let file_metadata = FileMetadata {
                len: metadata.len(),
                accessed: metadata.accessed().ok().and_then(system_time_to_u64),
                created: metadata.created().ok().and_then(system_time_to_u64),
                file_type: DirFileType::from_file_type(metadata.file_type()),
                modified: metadata.modified().ok().and_then(system_time_to_u64),
                read_only: metadata.permissions().readonly(),
        };


        let path = file.path();

        let file_name = match path.file_name() {
            None => {String::from("Name not found")}
            Some(name) => {name.to_string_lossy().to_string()}
        };


        Ok(FileInfo {
            file_name,
            file_metadata
        })
    }).collect();

    info
}

#[derive(Serialize, Clone, Eq, PartialEq)]
pub(crate) enum DirFileType {
    Dir, File, Symlink
}

impl DirFileType {
    pub(crate) fn from_file_type(file_type: FileType) -> Self {
        if file_type.is_file() { File } else if file_type.is_dir() { Dir } else { Symlink }
    }
}

#[derive(Serialize, Clone, Eq, PartialEq)]
pub(crate) struct FileMetadata {
    pub(crate) len: u64,
    pub(crate) accessed: Option<u64>,
    pub(crate) created: Option<u64>,
    pub(crate) file_type: DirFileType,
    pub(crate) modified: Option<u64>,
    pub(crate) read_only: bool,
}

#[derive(Serialize, Clone)]
pub(crate) struct FileInfo {
    pub(crate) file_name: String,
    pub(crate) file_metadata: FileMetadata
}

pub(crate) enum FileError {
    MetadataError(String),
    PathBuf(String),
    DirError(String)
}

fn system_time_to_u64(time: SystemTime) -> Option<u64> {
    time.duration_since(UNIX_EPOCH).ok().map(|d| d.as_secs())
}
