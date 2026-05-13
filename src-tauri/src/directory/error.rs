use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct FileError {
    pub(crate) error_type: FileErrorType,
    pub(crate) message: String
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) enum FileErrorType {
    Metadata,
    Path,
    Directory,
}

impl FileError {
    pub(crate) fn metadata(message: impl Into<String>) -> Self {
        Self {
            error_type: FileErrorType::Metadata,
            message: message.into(),
        }
    }

    pub(crate) fn path(message: impl Into<String>) -> Self {
        Self {
            error_type: FileErrorType::Path,
            message: message.into(),
        }
    }

    pub(crate) fn directory(message: impl Into<String>) -> Self {
        Self {
            error_type: FileErrorType::Directory,
            message: message.into(),
        }
    }
}


// Without this Tauri doesn't accept the error. Could I just return a string instead of this error? Yes, but I like the abstraction
impl fmt::Display for FileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}: {}", self.error_type, self.message)
    }
}