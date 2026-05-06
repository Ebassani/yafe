use std::fs;
use std::fs::{read_dir, Metadata};
use crate::directory::errors::FileError;
use crate::directory::types::{DirFileType, FileInfo, FileMetadata};
use crate::directory::utils::system_time_to_u64;

pub(crate) fn read_directory(dir_path: &str) -> Result<Vec<FileInfo>, FileError> {
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

pub(crate) fn read_metadata(path: &str) -> Result<FileMetadata, FileError> {
    let metadata = fs::symlink_metadata(path).map_err(|err| FileError::MetadataError(err.to_string()))?;

    Ok(fs_metadata_into_file_metadata(metadata))
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