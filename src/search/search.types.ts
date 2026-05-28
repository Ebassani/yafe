export interface IndexedEntry {
    path: string,
    name: string,
    kind: IndexedEntryKind
}

export type IndexedEntryKind = 'File' | 'Directory' | 'Symlink' | 'Other'