extern crate directories;
use std::path::Path;
use directories::UserDirs;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub(crate) enum UserDirType {
    Home, Desktop, Downloads, Documents, Pictures, Videos
}

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct UserDir {
    pub(crate) user_dir_type: UserDirType,
    pub(crate) dir_path: String
}

pub(crate) fn list_user_dirs() -> Vec<UserDir> {
    let mut user_directories: Vec<UserDir> = Vec::new();

    if let Some(user_dir) = UserDirs::new() {
        let mut push_to_vec = |dir: Option<&Path>, user_dir_type: UserDirType| {
            if let Some(directory) = dir {
                user_directories.push(UserDir {
                    user_dir_type,
                    dir_path: directory.to_string_lossy().to_string(),
                })
            }
        };

        push_to_vec(Some(user_dir.home_dir()), UserDirType::Home);
        push_to_vec(user_dir.desktop_dir(), UserDirType::Desktop);
        push_to_vec(user_dir.download_dir(), UserDirType::Downloads);
        push_to_vec(user_dir.document_dir(), UserDirType::Documents);
        push_to_vec(user_dir.picture_dir(), UserDirType::Pictures);
        push_to_vec(user_dir.video_dir(), UserDirType::Videos);
    }

    user_directories
}