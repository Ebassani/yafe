use std::fs::{read_dir};
use crate::directory::{dir_entry_into_file_info, DirFileType, FileError, FileErrorType, FileErrorWithPath};
use crate::search::indexer::{IndexedEntryKind, Indexer};

pub(crate) struct CrawlCoordinator {
    error_trace: Vec<FileErrorWithPath>
}

impl CrawlCoordinator {
    pub(crate) fn new() -> Self {
        Self {
            error_trace: Vec::new()
        }
    }
}

fn crawl(dir_path: &str, indexer: &Indexer, coordinator: &mut CrawlCoordinator) {
    let dir = match read_dir(dir_path).map_err(|message| FileError::directory(message.to_string())) {
        Ok(read_dir) => {read_dir}
        Err(err) => {
            let err_path = FileErrorWithPath::from_file_error_and_path(err, dir_path);
            coordinator.error_trace.push(err_path);

            return;
        }
    };
    
    for  dir_item  in dir {
        match dir_item {
            Ok(item) => {
                let file_info = match dir_entry_into_file_info(item) {
                    Ok(file_info) => {file_info}
                    Err(err) => {
                        let err_path = FileErrorWithPath::from_file_error_and_path(err, dir_path);
                        coordinator.error_trace.push(err_path);

                        continue;
                    }
                };

                let file_type = file_info.file_metadata.file_type;

                let is_dir = file_type == DirFileType::Dir;

                indexer.index_file(file_info.file_name, file_info.path.clone(), IndexedEntryKind::from(file_type));

                if is_dir {
                    crawl(&file_info.path, indexer, coordinator)
                }
            }
            Err(err) => {
                let file_error = FileError {
                    error_type: FileErrorType::Directory,
                    message: err.to_string()
                };
                let err_path = FileErrorWithPath::from_file_error_and_path(file_error, dir_path);
                coordinator.error_trace.push(err_path);

                continue;
            }
        }
    }
}