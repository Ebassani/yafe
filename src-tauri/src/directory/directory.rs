use std::path::Path;
use std::fs;
use std::fs::{read_dir, DirEntry, Metadata};
use std::mem::take;
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::ipc::Channel;
use crate::directory::{DirFileType, DirectoryStreamEvent, FileError, FileInfo, FileMetadata};

pub(crate) fn list(dir_path: String) -> Result<Vec<FileInfo>, FileError> {
    let dir = read_dir(dir_path).map_err(|message| FileError::directory(message.to_string()))?;

    let info: Result<Vec<FileInfo>, FileError> = dir
        .map(|dir_item| {
            let file = dir_item.map_err(|some_err| FileError::directory(some_err.to_string()))?;

            dir_entry_into_file_info(file)
        })
        .collect();

    info
}

pub(crate) fn dir_entry_into_file_info(file: DirEntry) -> Result<FileInfo, FileError> {
    let metadata = file
        .metadata()
        .map_err(|err| FileError::metadata(err.to_string()))?;

    let file_metadata = fs_metadata_into_file_metadata(metadata);

    let file_name = file.file_name().to_string_lossy().to_string();
    let path = file.path().to_string_lossy().to_string();

    Ok(FileInfo {
        path,
        file_name,
        file_metadata,
    })
}

const BATCH_SIZE: usize = 10;

pub(crate) fn list_streamed(dir_path: String, request_id: String, on_event: Channel<DirectoryStreamEvent>) -> Result<(), FileError> {
    on_event.send(DirectoryStreamEvent::Start {
        path: dir_path.clone(),
        request_id: request_id.clone()
    }).map_err(|message| FileError::directory(message.to_string()))?;

    let dir = read_dir(&dir_path).map_err(|message| FileError::directory(message.to_string()))?;

    let mut batch: Vec<FileInfo> = Vec::with_capacity(BATCH_SIZE);
    let mut sequence = 0;
    let mut total = 0;

    for dir_item in dir {
        let entry = match dir_item {
            Ok(entry) => entry,
            Err(err) => {
                on_event.send(DirectoryStreamEvent::Error {
                    request_id: request_id.clone(),
                    path: dir_path.clone(),
                    message: err.to_string()
                }).map_err(|send_err| FileError::directory(send_err.to_string()))?;
                continue
            },
        };

        let file_info = dir_entry_into_file_info(entry)?;
        batch.push(file_info);

        total += 1;

        if batch.len() >= BATCH_SIZE {
            let entries = take(&mut batch);

            on_event.send(DirectoryStreamEvent::Chunk {
                request_id: request_id.clone(),
                path: dir_path.clone(),
                entries,
                sequence,
            }).map_err(|error| FileError::directory(error.to_string()))?;

            sequence +=1;
        }

    }

    if !batch.is_empty() {
        on_event.send(DirectoryStreamEvent::Chunk {
            request_id: request_id.clone(),
            path: dir_path.clone(),
            entries: batch,
            sequence,
        }).map_err(|error| FileError::directory(error.to_string()))?;
    }

    on_event.send(DirectoryStreamEvent::Complete {
        request_id,
        path: dir_path,
        total,
    }).map_err(|message| FileError::directory(message.to_string()))?;

    Ok(())
}

pub(crate) fn read(path: &str) -> Result<FileInfo, FileError> {
    let metadata = fs::symlink_metadata(path).map_err(|err| FileError::metadata(err.to_string()))?;

    let file_name = Path::new(path)
        .file_name()
        .ok_or_else(|| FileError::path("Invalid path"))?
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