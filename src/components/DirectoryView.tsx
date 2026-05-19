import {DirectoryEvent, FileInfo, listDirectoryStream} from "../directory";
import {useEffect, useState} from "react";
import {Channel} from "@tauri-apps/api/core";

interface Props {
    path: string;
}

export default function DirectoryView({path}: Props) {
    const [files, setFiles] = useState<FileInfo[]>([]);
    const [_, setError] = useState<string | undefined>(undefined);

    const onEvent = new Channel<DirectoryEvent>();

    const requestId = 'someid';

    onEvent.onmessage = (event) => {
        if (event.data.requestId !== requestId) return;

        if (event.event=== 'start') {
            console.log('started')
            return;
        }
        if (event.event === 'chunk') {
            setFiles([...files, ...event.data.entries] )
        }
        if (event.event=== 'error') {
            console.log(event)
            return;
        }
        if (event.event=== 'complete') {
            console.log('compete')
            return;
        }
    }

    useEffect(() => {
        (async () => {
            try {
                await listDirectoryStream(path, requestId, onEvent);
            } catch (e) {
                setError(String(e));
            }
        })()
    }, [])

    return (
        <div>
            {files.map(file => (
                <div>
                    <span>{file.file_name}</span>
                </div>
            ))}
        </div>
    )
}