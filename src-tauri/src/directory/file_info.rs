use std::fs::FileType;
use serde::{Serialize};

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

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase",tag="event", content="data")]
pub(crate) enum DirectoryStreamEvent {
    Start {
        request_id: String,
        path: String
    },
    Chunk {
        request_id: String,
        path: String,
        entries: Vec<FileInfo>,
        sequence: u64
    },
    Error {
        request_id: String,
        path: String,
        message: String
    },
    Complete {
        request_id: String,
        path: String,
        total: usize
    }
}