use std::fs::Metadata;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::directory::types::{DirFileType, FileMetadata};

pub(crate) fn system_time_to_u64(time: SystemTime) -> Option<u64> {
    time.duration_since(UNIX_EPOCH).ok().map(|d| d.as_secs())
}

pub(crate) fn fs_metadata_into_file_metadata(metadata: Metadata) -> FileMetadata {
    FileMetadata {
        len: metadata.len(),
        accessed: metadata.accessed().ok().and_then(system_time_to_u64),
        created: metadata.created().ok().and_then(system_time_to_u64),
        file_type: DirFileType::from_file_type(metadata.file_type()),
        modified: metadata.modified().ok().and_then(system_time_to_u64),
        read_only: metadata.permissions().readonly(),
    }
}