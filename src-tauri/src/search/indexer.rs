use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::directory::DirFileType;

pub(crate) trait Indexer: Send + Sync {
    fn index_file(&self, name: String, path: String, indexed_entry_kind: IndexedEntryKind) -> FileId;

    fn search_file_candidates(&self, target_name: &str) -> Vec<SearchCandidate>;

    fn get_indexed_entries(&self, ids: &[FileId]) -> Vec<IndexedEntry>;

    fn get_indexed_entries_by_candidates(&self, ids: &[SearchCandidate]) -> Vec<IndexedEntry>;

    fn search_and_get_indexed_entries(&self, target_name: &str) -> Vec<IndexedEntry>;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct SearchCandidate {
    pub(crate) file_id: FileId,
    pub(crate) score: u32
}

/// Can't say I got this idea on my own, so credits to some AI model.
/// I was having problems turning everything async as essentially, if I had just turned the main postings: HashMap<GramId, Vec<FileId>> into a Mutex it would work almost as a threaded linear function.
/// Every time any thread wants the Mutex, it gets locked, so no other can use it(Perfect >:( ). SO the idea is to turn it into a Vec<Mutex<ShardedPostings>>
#[derive(Default)]
pub(crate) struct ShardedPostings {
    pub(crate) postings: HashMap<GramId, Vec<FileId>>
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub(crate) struct FileId(pub(crate) u32);

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub(crate) struct GramId(pub(crate) u32);

impl GramId {
    pub(crate) fn from_bytes(bytes: [u8; 3]) -> Self {
        GramId(u32::from_be_bytes([0, bytes[0], bytes[1], bytes[2]]))
    }

    pub fn as_usize(self) -> usize {
        self.0 as usize
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct IndexedEntry {
    pub(crate) path: String,
    pub(crate) name: String,
    pub(crate) kind: IndexedEntryKind,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
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