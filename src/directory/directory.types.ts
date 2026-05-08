export interface FileInfo {
    path: string,
    file_name: string,
    file_metadata: FileMetadata
}

export interface FileMetadata {
    len: number,
    accessed: number | undefined,
    created: number | undefined,
    file_type: FileType,
    modified: number | undefined,
    read_only: boolean
}

export type FileType = 'Dir' | 'File' | 'Symlink';

export interface UserDir {
    dir_path: string,
    user_dir_type: UserDirType
}

export type UserDirType = 'Home' | 'Desktop' | 'Downloads' | 'Documents' | 'Pictures' | 'Videos';