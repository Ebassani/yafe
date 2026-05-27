use std::fs::{read_dir};
use crate::directory::{dir_entry_into_file_info, DirFileType, FileError};

fn crawl(dir_path: &str) {
    let dir = match read_dir(dir_path).map_err(|message| FileError::directory(message.to_string())) {
        Ok(some) => {some}
        Err(_) => {return;}
    };
    
    for  (_, dir_item)  in dir.enumerate() {
        match dir_item {
            Ok(item) => {
                let file_info = match dir_entry_into_file_info(item) {
                    Ok(some) => {some}
                    Err(_) => {continue}
                };
                println!("{}", file_info.file_name);
                if file_info.file_metadata.file_type == DirFileType::Dir {
                    crawl(&file_info.path)
                }
            }
            Err(_) => {}
        }
    }
}