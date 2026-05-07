use std::fs::FileType;
use serde::Serialize;

#[derive(Serialize, Clone, Eq, PartialEq)]
pub(crate) enum DirFileType {
    Dir,
    File,
    Symlink,
}

impl DirFileType {
    pub(crate) fn from_file_type(file_type: FileType) -> Self {
        if file_type.is_file() {
            DirFileType::File
        } else if file_type.is_dir() {
            DirFileType::Dir
        } else {
            DirFileType::Symlink
        }
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
    pub(crate) path: String,
    pub(crate) file_name: String,
    pub(crate) file_metadata: FileMetadata,
}