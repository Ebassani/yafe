import {UserDir} from "./directory.types.ts";
import {safeInvoke} from "../safeInvoke.ts";

export async function listUserDirectories(): Promise<UserDir[]> {
    return safeInvoke<UserDir[]>('list_user_directories');
}