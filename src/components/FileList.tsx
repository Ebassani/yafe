import {AlertTriangle, FolderOpen, Loader2} from "lucide-react";
import {FileInfo} from "../directory";
import {FileRow} from "./FileRow";

interface Props {
    files: FileInfo[];
    loading: boolean;
    error?: string;
}

export function FileList({files, loading, error}: Props) {
    return (
        <div className="flex min-h-0 flex-1 flex-col bg-main-bg">
            {error && (
                <div className="mx-4 mt-3 flex items-center gap-2 rounded-md border border-danger-line bg-danger-soft px-3 py-2 text-sm text-danger-text">
                    <AlertTriangle className="h-4 w-4 shrink-0"/>
                    <span className="min-w-0 truncate">{error}</span>
                </div>
            )}

            <div className="grid h-9 shrink-0 grid-cols-[minmax(12rem,1fr)_6rem_7rem_10rem] items-center gap-3 border-b border-line-muted bg-panel-bg px-4 text-xs font-medium uppercase tracking-wide text-text-faint">
                <div>Name</div>
                <div>Type</div>
                <div>Size</div>
                <div>Modified</div>
            </div>

            <div className="min-h-0 flex-1 overflow-auto">
                {files.length > 0 ? (
                    files.map((file) => <FileRow key={file.path} file={file}/>)
                ) : (
                    <EmptyFileList loading={loading}/>
                )}
            </div>
        </div>
    );
}

function EmptyFileList({loading}: { loading: boolean }) {
    return (
        <div className="flex h-full min-h-64 items-center justify-center px-6 text-center">
            <div className="flex max-w-sm flex-col items-center gap-3 text-text-muted">
                <span className="flex h-11 w-11 items-center justify-center rounded-md border border-line-soft bg-panel-bg text-text-faint">
                    {loading ? (
                        <Loader2 className="h-5 w-5 animate-spin text-accent-strong"/>
                    ) : (
                        <FolderOpen className="h-5 w-5"/>
                    )}
                </span>
                <div>
                    <div className="text-sm font-medium text-text-soft">
                        {loading ? "Loading folder" : "Folder is empty"}
                    </div>
                    <div className="mt-1 text-xs text-text-faint">
                        {loading ? "Loading..." : "No files or folders were found."}
                    </div>
                </div>
            </div>
        </div>
    );
}
