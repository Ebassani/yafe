use std::fs::{read_dir, FileType};
use std::time::{SystemTime, UNIX_EPOCH};
use serde::Serialize;
use crate::directory::DirFileType::{Dir, File, Symlink};

pub(crate) fn read_directory(path: &str) -> Result<Vec<FileInfo>, FileError> {
    let dir = read_dir(path).map_err(|message| FileError::DirError(message.to_string()))?;

    let mut info: Vec<FileInfo> = Vec::new();
    
    let _ = dir.filter_map(|dir_item| dir_item.ok().map(|file| {
        let metadata = file.metadata().map_err(|err| FileError::MetadataError(err.to_string())).ok().map(|file_metadata|{
            FileMetadata {
                len: file_metadata.len(),
                accessed: file_metadata.accessed().ok().and_then(system_time_to_u64),
                created: file_metadata.created().ok().and_then(system_time_to_u64),
                file_type: DirFileType::from_file_type(file_metadata.file_type()),
                modified: file_metadata.modified().ok().and_then(system_time_to_u64),
                permissions: 0,
                creation_time: 0,
                file_attributes: 0,
                file_size: 0,
                last_access_time: 0,
                last_write_time: 0,
            }
        });


        let file = FileInfo {
            file_metadata: metadata
        };

        info.push(file)
    }));

    Ok(info)
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
    pub(crate) permissions: u64,
    pub(crate) creation_time: u64,
    pub(crate) file_attributes: u64,
    pub(crate) file_size: u64,
    pub(crate) last_access_time: u64,
    pub(crate) last_write_time: u64,
}

#[derive(Serialize, Clone)]
pub(crate) struct FileInfo {
    pub(crate) file_metadata: Option<FileMetadata>
}

pub(crate) enum FileError {
    MetadataError(String),
    PathBuf(String),
    DirError(String)
}

fn system_time_to_u64(time: SystemTime) -> Option<u64> {
    time.duration_since(UNIX_EPOCH).ok().map(|d| d.as_secs())
}
