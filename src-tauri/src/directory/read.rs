use std::path::Path;
use std::fs;
use std::fs::read_dir;
use crate::directory::errors::FileError;
use crate::directory::types::{FileInfo};
use crate::directory::utils;


pub(crate) fn read_directory(dir_path: &str) -> Result<Vec<FileInfo>, FileError> {
    let dir = read_dir(dir_path).map_err(|message| FileError::DirError(message.to_string()))?;

    let info: Result<Vec<FileInfo>, FileError> = dir
        .map(|dir_item| {
            let file = dir_item.map_err(|some_err| FileError::DirError(some_err.to_string()))?;

            let metadata = file
                .metadata()
                .map_err(|err| FileError::MetadataError(err.to_string()))?;

            let file_metadata = utils::fs_metadata_into_file_metadata(metadata);

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

pub(crate) fn read_metadata(path: &str) -> Result<FileInfo, FileError> {
    let metadata = fs::symlink_metadata(path).map_err(|err| FileError::MetadataError(err.to_string()))?;

    let file_name = Path::new(path)
        .file_name()
        .ok_or_else(|| FileError::PathBuf("Invalid path".into()))?
        .to_string_lossy()
        .to_string();


    Ok(FileInfo {
        path: path.to_string(),
        file_name,
        file_metadata: utils::fs_metadata_into_file_metadata(metadata),
    })
}