use std::collections::HashMap;
use crate::directory::DirFileType;

pub(crate) struct Indexer{
    pub(crate) postings: HashMap<GramId, Vec<FileId>>,
    pub(crate) files: HashMap<FileId, IndexedEntry>,
    pub(crate) file_grams: HashMap<FileId, Vec<GramId>>
}

pub(crate) struct FileId(pub(crate) u32);

pub(crate) struct GramId(pub(crate) u32);

impl GramId {
    pub(crate) fn from_bytes(bytes: [u8; 3]) -> Self {
        GramId(u32::from_be_bytes([0, bytes[0], bytes[1], bytes[2]]))
    }
}

pub(crate) struct IndexedEntry {
    path: String,
    name: String,
    kind: IndexedEntryKind,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum IndexedEntryKind {
    File,
    Directory,
    Symlink,
    Other,
}

impl From<DirFileType> for IndexedEntryKind {
    fn from(value: DirFileType) -> Self {
        match value {
            DirFileType::File => Self::File,
            DirFileType::Dir => Self::Directory,
            DirFileType::Symlink => Self::Symlink,
        }
    }
}