use std::collections::HashMap;
use std::sync::{Mutex, RwLock};
use std::sync::atomic::{AtomicU32, Ordering};
use crate::directory::DirFileType;

/// This file will be a nightmare, I was researching about search types and I found one called trigram search.
/// So the idea is to separate all the files/directories in grams from their names, let's say for `load`, we do the split with the grams `loa` and `oad`.
/// To make lookup fast we put the gram in a hashmap alongside all the files that have that gram.
/// I thought that saving a byte array as key on the hashmap was not ideal, so I found out I can just turn the bytes into an u32, not the biggest optimization ever but should work better.
///
/// # Async
/// I also realized this has to be async as I want to make my file crawler async to make it faster. So now tha is also a slight problem
pub(crate) struct Indexer{
    shards: Vec<Mutex<ShardedPostings>>,
    files: RwLock<HashMap<FileId, IndexedEntry>>,
    file_grams: RwLock<HashMap<FileId, Vec<GramId>>>,
    next_file_id: AtomicU32
}

const DEFAULT_SHARD_COUNT: usize = 64;

impl Indexer {
    pub(crate) fn new() -> Self {
        Self::with_shard_count(DEFAULT_SHARD_COUNT)
    }

    pub(crate) fn with_shard_count(shard_count: usize) -> Self {
        assert!(shard_count > 0);

        Self {
            shards: (0..shard_count)
                .map(|_| Mutex::new(ShardedPostings::default()))
                .collect(),
            files: Default::default(),
            file_grams: Default::default(),
            next_file_id: AtomicU32::new(0),
        }
    }

    pub(crate) fn index_file(&self, name: String, path: String, indexed_entry_kind: IndexedEntryKind) -> FileId {
        let file_id = self.next_file_id();

        let file_entry = IndexedEntry {
            path,
            name: name.clone(),
            kind: indexed_entry_kind,
        };

        self.files.write().unwrap().insert(file_id, file_entry);

        let grams = Self::grams_from_text(&name);

        grams.iter().for_each(|&gram| {
            self.insert_file_gram(gram, file_id)
        });

        self.file_grams.write().unwrap().insert(file_id, grams);

        file_id
    }

    pub(crate) fn search_file_candidates(&self, target_name: &str) -> Vec<SearchCandidate> {
        let grams = Self::grams_from_text(target_name);

        let mut scores: HashMap<FileId, u32> = HashMap::new();

        grams.iter().for_each(|&gram| {
           let index = self.shard_index(gram);

            if let Some(ids) = self.shards[index].lock().unwrap().postings.get(&gram) {
                ids.iter().for_each(|&file_id| {
                    *scores.entry(file_id).or_default() +=1;
                });
            }
        });

        let minimum_score = ((grams.len() as u32 + 1) / 2).max(1);

        let mut candidates: Vec<SearchCandidate> = scores
            .into_iter()
            .filter(|(_, score)| *score >= minimum_score)
            .map(|(file_id, score)| SearchCandidate { file_id, score })
            .collect();

        candidates.sort_unstable_by(|left, right| {
            right
                .score
                .cmp(&left.score)
                .then_with(|| left.file_id.cmp(&right.file_id))
        });

        candidates
    }

    pub(crate) fn get_indexed_entries(&self, ids: &[FileId]) -> Vec<IndexedEntry> {
        let files = self.files.read().unwrap();

        ids.iter().filter_map(|id| files.get(id).cloned()).collect()
    }

    fn insert_file_gram(&self, gram_id: GramId, file_id: FileId) {
        let index = self.shard_index(gram_id);
        let mut shard = self.shards[index].lock().unwrap();

        shard.postings.entry(gram_id).or_default().push(file_id);
    }

    fn next_file_id(&self) -> FileId {
        let id = self.next_file_id.fetch_add(1, Ordering::Relaxed);

        assert_ne!(id, u32::MAX, "file id allocator exhausted");

        FileId(id)
    }

    /// This is here to make sure the index used to find a gram is always the same. So, in case a trigram has to be added to the shards, the shard index can be calculated, same thing if it is to retrieve the gram
    fn shard_index(&self, gram: GramId) -> usize {
        gram.as_usize() % self.shards.len()
    }

    /// If you are reading this and wondering "Why the filter?". I'm trying to make the trigrams more forgiving, so `my_file` and `my file` both become `myfile`,
    /// making the trigrams for both `myf`, `yfi`, `fil` and `ile`. In the end, that is just to help some people(like me) who don't always remember the proper casing and format of their file names
    fn normalize(text: &str) -> String {
        text.to_lowercase()
            .chars()
            .filter(|character| character.is_alphanumeric())
            .collect()
    }

    fn grams_from_text(text: &str) -> Vec<GramId> {
        let normalized = Self::normalize(text);
        let bytes = normalized.as_bytes();

        if bytes.len() < 3 {
            return Vec::new();
        }

        let mut grams = Vec::with_capacity(bytes.len() - 2);

        for window in bytes.windows(3) {
            grams.push(GramId::from_bytes([window[0], window[1], window[2]]));
        }

        // This sort should work for now, I added Ord to GramId
        grams.sort_unstable();
        grams.dedup();

        grams
    }
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

#[derive(Clone)]
pub(crate) struct IndexedEntry {
    pub(crate) path: String,
    pub(crate) name: String,
    pub(crate) kind: IndexedEntryKind,
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