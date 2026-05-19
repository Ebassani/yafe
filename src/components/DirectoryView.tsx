import {DirectoryEvent, FileInfo, listDirectoryStream} from "../directory";
import {useEffect, useRef, useState} from "react";
import {Channel} from "@tauri-apps/api/core";
import {DirectoryToolbar} from "./DirectoryToolbar";
import {FileList} from "./FileList";

interface Props {
    path: string;
}

export default function DirectoryView({path}: Props) {
    const [files, setFiles] = useState<FileInfo[]>([]);
    const [error, setError] = useState<string | undefined>(undefined);
    const [isLoading, setIsLoading] = useState(false);
    const activeRequestIdRef = useRef<string | undefined>(undefined);

    useEffect(() => {
        const requestId = crypto.randomUUID();
        const onEvent = new Channel<DirectoryEvent>();

        activeRequestIdRef.current = requestId;
        setFiles([]);
        setError(undefined);
        setIsLoading(true);

        onEvent.onmessage = (event) => {
            if (activeRequestIdRef.current !== event.data.requestId) {
                return;
            }

            if (event.event === 'start') {
                return;
            }

            if (event.event === 'chunk') {
                setFiles((currentFiles) => [...currentFiles, ...event.data.entries]);
                return;
            }

            if (event.event === 'error') {
                setError(event.data.message);
                return;
            }

            if (event.event === 'complete') {
                setIsLoading(false);
            }
        };

        void listDirectoryStream(path, requestId, onEvent)
            .catch((e) => {
                if (activeRequestIdRef.current !== requestId) {
                    return;
                }

                setError(getErrorMessage(e));
            })
            .finally(() => {
                if (activeRequestIdRef.current === requestId) {
                    setIsLoading(false);
                }
            });

        return () => {
            if (activeRequestIdRef.current === requestId) {
                activeRequestIdRef.current = undefined;
            }
        };
    }, [path]);

    return (
        <section className="flex h-full min-h-0 flex-col bg-main-bg text-text-main">
            <DirectoryToolbar path={path} fileCount={files.length} loading={isLoading}/>
            <FileList files={files} loading={isLoading} error={error}/>
        </section>
    )
}

function getErrorMessage(error: unknown): string {
    if (typeof error === 'string') {
        return error;
    }

    if (error && typeof error === 'object' && 'message' in error) {
        return String(error.message);
    }

    return String(error);
}
