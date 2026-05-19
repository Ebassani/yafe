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

export type FileErrorType = 'Metadata' | 'Directory' | 'Path';

export interface FileError {
    errorType: FileErrorType,
    message: string
}

export type DirectoryEvent =
    {
        event: 'start',
        data: {
            requestId: string,
            path: string
        }
    } |
    {
        event: 'chunk'
        data: {
            requestId: string,
            path: string,
            entries: FileInfo[],
            sequence: number
        }
    } |
    {
        event: 'error',
        data: {
            requestId: string,
            path: string,
            message: string
        }
    } |
    {
        event: 'complete',
        data: {
            requestId: string,
            path: string,
            total: bigint
        }
    }