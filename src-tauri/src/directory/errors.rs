pub(crate) enum FileError {
    MetadataError(String),
    PathBuf(String),
    DirError(String),
}