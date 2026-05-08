import {invoke} from "@tauri-apps/api/core";
import {UserDir} from "./directory.types.ts";

export async function listUserDirectories(): Promise<UserDir[]> {
    const response = await invoke('list_user_directories');
    return response as UserDir[];
}