use std::fs::{read_dir};
use std::sync::{Arc, Mutex};
use crate::directory::{dir_entry_into_file_info, DirFileType, FileError, FileErrorType, FileErrorWithPath};
use crate::queue::Queue;
use crate::search::Indexer;
use crate::search::indexer::{IndexedEntryKind};

pub(crate) struct CrawlCoordinator {
    error_trace: Mutex<Vec<FileErrorWithPath>>,
    queue: Arc<Queue<String>>
}

const THREAD_AMOUNT: usize = 8;

impl CrawlCoordinator {
    pub(crate) fn new() -> Self {
        Self {
            error_trace: Mutex::new(Vec::new()),
            queue: Arc::new(Queue::new()),
        }
    }

    pub(crate) async fn crawl_and_index(self: Arc<Self>, root_paths: Vec<String>, indexer: Arc<dyn Indexer>) {
        for path in root_paths {
            self.queue.push(path);
        }

        Arc::clone(&self.queue).run_blocking(THREAD_AMOUNT, {
            let indexer = Arc::clone(&indexer);
            let coordinator = Arc::clone(&self);

            move |dir_path| {
                coordinator.crawl_one_directory(&dir_path, indexer.as_ref());
            }
        }).await;

    }

    fn crawl_one_directory(&self, dir_path: &str, indexer: &dyn Indexer) {
        let dir = match read_dir(dir_path).map_err(|message| FileError::directory(message.to_string())) {
            Ok(read_dir) => {read_dir}
            Err(err) => {
                let err_path = FileErrorWithPath::from_file_error_and_path(err, dir_path);
                self.error_trace.lock().unwrap().push(err_path);

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
                            self.error_trace.lock().unwrap().push(err_path);

                            continue;
                        }
                    };

                    let file_type = file_info.file_metadata.file_type;

                    let is_dir = file_type == DirFileType::Dir;

                    indexer.index_file(file_info.file_name, file_info.path.clone(), IndexedEntryKind::from(file_type));

                    if is_dir {
                        self.queue.push(file_info.path)
                    }
                }
                Err(err) => {
                    let file_error = FileError {
                        error_type: FileErrorType::Directory,
                        message: err.to_string()
                    };
                    let err_path = FileErrorWithPath::from_file_error_and_path(file_error, dir_path);
                    self.error_trace.lock().unwrap().push(err_path);

                    continue;
                }
            }
        }
    }
}