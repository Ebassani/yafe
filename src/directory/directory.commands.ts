import {DirectoryEvent, FileInfo, UserDir} from "./directory.types.ts";
import {safeInvoke} from "../safeInvoke.ts";
import {Channel} from "@tauri-apps/api/core";

/**
 * List the default user directories.
 * A quick way to get the paths for things such as `Downloads`, `Documents`, and so on.
 */
export async function listUserDirectories(): Promise<UserDir[]> {
    return safeInvoke<UserDir[]>('list_user_directories');
}

/**
 * Lists the contents of a given directory
 * @param path - The path of the directory to list the contents of
 */
export async function listDirectory(path: string): Promise<FileInfo[]> {
    return safeInvoke<FileInfo[]>('list_directory', {path});
}

export async function listDirectoryStream(path: string, requestId: string, onEvent: Channel<DirectoryEvent>): Promise<void> {
    return safeInvoke<void>('list_directory_stream', {path, requestId, onEvent})
}
