import {invoke, type InvokeArgs} from "@tauri-apps/api/core";

export async function safeInvoke<T>(cmd: string, args?: InvokeArgs): Promise<T> {
    try {
        return await invoke<T>(cmd, args);
    } catch (e) {
        throw JSON.parse(String(e));
    }
}