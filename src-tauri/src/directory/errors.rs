use std::fmt;
use tauri::ipc::InvokeError;

pub(crate) enum FileError {
    MetadataError(String),
    PathBuf(String),
    DirError(String),
}

impl fmt::Display for FileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FileError::MetadataError(e) => write!(f, "Metadata error: {}", e),
            FileError::PathBuf(e) => write!(f, "Path error: {}", e),
            FileError::DirError(e) => write!(f, "Directory error: {}", e),
        }
    }
}

impl From<FileError> for InvokeError {
    fn from(err: FileError) -> Self {
        InvokeError::from(err.to_string())
    }
}