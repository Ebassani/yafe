use std::ffi::{OsString};
use std::fs::{read_dir, FileType};
use std::os::windows::prelude::MetadataExt;
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
                read_only: file_metadata.permissions().readonly(),
                creation_time: file_metadata.creation_time(),
                file_attributes: file_metadata.file_attributes(),
                file_size: file_metadata.file_size(),
                last_access_time: file_metadata.last_access_time(),
                last_write_time: file_metadata.last_write_time(),
            }
        });

        let path = file.path();

        let file_name = match path.file_name() {
            None => {OsString::from("Name not found")}
            Some(name) => {name.to_os_string()}
        };


        let file = FileInfo {
            file_name,
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
    pub(crate) read_only: bool,
    pub(crate) creation_time: u64,
    pub(crate) file_attributes: u32,
    pub(crate) file_size: u64,
    pub(crate) last_access_time: u64,
    pub(crate) last_write_time: u64,
}

#[derive(Serialize, Clone)]
pub(crate) struct FileInfo {
    pub(crate) file_name: OsString,
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
