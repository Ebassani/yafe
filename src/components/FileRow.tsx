import {File as FileIcon, Folder, Link2} from "lucide-react";
import {FileInfo, FileType} from "../directory";

interface Props {
    file: FileInfo;
}

const fileTypeLabels: Record<FileType, string> = {
    Dir: "Folder",
    File: "File",
    Symlink: "Link",
};

export function FileRow({file}: Props) {
    const fileType = file.file_metadata.file_type;
    const Icon = getFileIcon(fileType);

    return (
        <div className="grid min-h-10 grid-cols-[minmax(12rem,1fr)_6rem_7rem_10rem] items-center gap-3 border-b border-line-muted px-4 text-sm text-text-soft transition-colors hover:bg-panel-soft">
            <div className="flex min-w-0 items-center gap-3">
                <span className={getIconClassName(fileType)}>
                    <Icon className="h-4 w-4"/>
                </span>
                <span className="truncate font-medium text-text-main">{file.file_name}</span>
            </div>
            <div className="truncate text-xs text-text-muted">{fileTypeLabels[fileType]}</div>
            <div className="truncate text-xs tabular-nums text-text-muted">{formatSize(file)}</div>
            <div className="truncate text-xs tabular-nums text-text-muted">
                {formatTimestamp(file.file_metadata.modified)}
            </div>
        </div>
    );
}

function getFileIcon(fileType: FileType) {
    if (fileType === "Dir") {
        return Folder;
    }

    if (fileType === "Symlink") {
        return Link2;
    }

    return FileIcon;
}

function getIconClassName(fileType: FileType): string {
    const baseClassName = "flex h-8 w-8 shrink-0 items-center justify-center rounded-md border";

    if (fileType === "Dir") {
        return `${baseClassName} border-accent-muted bg-accent-soft text-accent-strong`;
    }

    if (fileType === "Symlink") {
        return `${baseClassName} border-warn-line bg-warn-soft text-warn-text`;
    }

    return `${baseClassName} border-line-soft bg-panel-bg text-text-muted`;
}

function formatSize(file: FileInfo): string {
    if (file.file_metadata.file_type === "Dir") {
        return "--";
    }

    const bytes = file.file_metadata.len;
    const units = ["B", "KB", "MB", "GB", "TB"];
    let value = bytes;
    let unitIndex = 0;

    while (value >= 1024 && unitIndex < units.length - 1) {
        value /= 1024;
        unitIndex += 1;
    }

    const precision = value >= 10 || unitIndex === 0 ? 0 : 1;
    return `${value.toFixed(precision)} ${units[unitIndex]}`;
}

function formatTimestamp(timestamp?: number): string {
    if (timestamp === undefined) {
        return "--";
    }

    return new Intl.DateTimeFormat(undefined, {
        dateStyle: "medium",
        timeStyle: "short",
    }).format(new Date(timestamp * 1000));
}
