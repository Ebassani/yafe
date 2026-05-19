import {ChevronRight, FolderOpen, Loader2} from "lucide-react";

interface Props {
    path: string;
    fileCount: number;
    loading: boolean;
}

export function DirectoryToolbar({path, fileCount, loading}: Props) {
    const segments = splitPath(path);

    return (
        <div
            className="flex h-14 shrink-0 items-center justify-between gap-3 border-b border-line-soft bg-panel-bg px-4">
            <div className="flex min-w-0 items-center gap-3">
                <FolderOpen className="h-6 w-6 text-accent-strong"/>
                <div className="min-w-0">
                    <div className="flex min-w-0 items-center overflow-hidden text-sm text-text-soft">
                        {segments.map((segment, index) => (
                            <span key={`${segment}-${index}`} className="flex min-w-0 items-center">
                                {index > 0 && (
                                    <ChevronRight className="mx-1 h-3.5 w-3.5 shrink-0 text-text-faint"/>
                                )}
                                <span
                                    className={[
                                        "truncate",
                                        index === segments.length - 1 ? "font-medium text-text-main" : "text-text-muted",
                                    ].join(" ")}
                                >
                                    {segment}
                                </span>
                            </span>
                        ))}
                    </div>
                </div>
            </div>

            <div className="flex shrink-0 items-center gap-2 text-xs text-text-muted">
                {loading && <Loader2 className="h-3.5 w-3.5 animate-spin text-accent-strong"/>}
                <span>{fileCount} items</span>
            </div>
        </div>
    );
}

function splitPath(path: string): string[] {
    const normalizedPath = path.replace(/\\/g, "/");
    const parts = normalizedPath.split("/").filter(Boolean);

    if (parts.length === 0) {
        return [path];
    }

    return parts;
}
