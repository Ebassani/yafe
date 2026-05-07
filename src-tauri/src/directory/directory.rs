use std::path::Path;
use std::fs;
use std::fs::{read_dir, Metadata};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::directory::{DirFileType, FileError, FileInfo, FileMetadata};

pub(crate) fn list(dir_path: &str) -> Result<Vec<FileInfo>, FileError> {
    let dir = read_dir(dir_path).map_err(|message| FileError::DirError(message.to_string()))?;

    let info: Result<Vec<FileInfo>, FileError> = dir
        .map(|dir_item| {
            let file = dir_item.map_err(|some_err| FileError::DirError(some_err.to_string()))?;

            let metadata = file
                .metadata()
                .map_err(|err| FileError::MetadataError(err.to_string()))?;

            let file_metadata = fs_metadata_into_file_metadata(metadata);

            let file_name = file.file_name().to_string_lossy().to_string();
            let path = file.path().to_string_lossy().to_string();

            Ok(FileInfo {
                path,
                file_name,
                file_metadata,
            })
        })
        .collect();

    info
}

pub(crate) fn read(path: &str) -> Result<FileInfo, FileError> {
    let metadata = fs::symlink_metadata(path).map_err(|err| FileError::MetadataError(err.to_string()))?;

    let file_name = Path::new(path)
        .file_name()
        .ok_or_else(|| FileError::PathBuf("Invalid path".into()))?
        .to_string_lossy()
        .to_string();


    Ok(FileInfo {
        path: path.to_string(),
        file_name,
        file_metadata: fs_metadata_into_file_metadata(metadata),
    })
}

pub(crate) fn system_time_to_u64(time: SystemTime) -> Option<u64> {
    time.duration_since(UNIX_EPOCH).ok().map(|d| d.as_secs())
}

fn fs_metadata_into_file_metadata(metadata: Metadata) -> FileMetadata {
    FileMetadata {
        len: metadata.len(),
        accessed: metadata.accessed().ok().and_then(system_time_to_u64),
        created: metadata.created().ok().and_then(system_time_to_u64),
        file_type: DirFileType::from_file_type(metadata.file_type()),
        modified: metadata.modified().ok().and_then(system_time_to_u64),
        read_only: metadata.permissions().readonly(),
    }
}