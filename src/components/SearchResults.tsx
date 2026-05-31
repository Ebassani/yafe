import {type ReactNode, useEffect, useState} from "react";
import {File as FileIcon, FileQuestion, Folder, Link2, Loader2, Search, SearchX} from "lucide-react";
import {IndexedEntry, IndexedEntryKind, searchIndex} from "../search";
import {useDebouncedValue} from "../hooks/useDebouncedValue";

interface Props {
    query: string;
    onOpenDirectory: (path: string) => void;
}

const MAX_RESULTS = 200;

export function SearchResults({query, onOpenDirectory}: Props) {
    const debouncedQuery = useDebouncedValue(query.trim(), 150);
    const [results, setResults] = useState<IndexedEntry[]>([]);
    const [isLoading, setIsLoading] = useState(false);
    const [error, setError] = useState<string | undefined>(undefined);

    useEffect(() => {
        if (debouncedQuery.length < 2) {
            setResults([]);
            setIsLoading(false);
            setError(undefined);
            return;
        }

        let cancelled = false;
        setIsLoading(true);
        setError(undefined);

        searchIndex(debouncedQuery)
            .then((entries) => {
                if (cancelled) return;
                setResults(entries.slice(0, MAX_RESULTS));
            })
            .catch((e) => {
                if (cancelled) return;
                setError(extractMessage(e));
                setResults([]);
            })
            .finally(() => {
                if (cancelled) return;
                setIsLoading(false);
            });

        return () => {
            cancelled = true;
        };
    }, [debouncedQuery]);

    if (debouncedQuery.length < 2) {
        return <SearchHint query={query}/>;
    }

    if (error) {
        return (
            <CenteredState
                icon={<FileQuestion className="h-5 w-5"/>}
                title="Search failed"
                subtitle={error}
                tone="danger"
            />
        );
    }

    if (isLoading && results.length === 0) {
        return (
            <CenteredState
                icon={<Loader2 className="h-5 w-5 animate-spin"/>}
                title="Searching the index"
                subtitle={`Looking for "${debouncedQuery}" across indexed locations.`}
            />
        );
    }

    if (results.length === 0) {
        return (
            <CenteredState
                icon={<SearchX className="h-5 w-5"/>}
                title="No matches"
                subtitle={`Nothing found for "${debouncedQuery}". Try a different term.`}
            />
        );
    }

    return (
        <section className="flex h-full min-h-0 flex-col bg-main-bg">
            <div className="flex h-14 shrink-0 items-center justify-between gap-3 border-b border-line-soft bg-panel-bg px-6">
                <div className="flex min-w-0 items-center gap-3">
                    <span className="flex h-7 w-7 items-center justify-center rounded-md border border-accent-muted bg-accent-soft text-accent-strong">
                        <Search className="h-4 w-4"/>
                    </span>
                    <div className="min-w-0">
                        <div className="truncate text-sm font-semibold text-text-main">
                            Search results
                        </div>
                        <div className="text-[11px] uppercase tracking-wide text-text-faint">
                            {results.length} matches for "{debouncedQuery}"
                            {results.length === MAX_RESULTS && " - capped at 200"}
                        </div>
                    </div>
                </div>
                {isLoading && (
                    <Loader2 className="h-4 w-4 animate-spin text-accent-strong"/>
                )}
            </div>

            <div className="min-h-0 flex-1 overflow-auto px-3 py-3">
                <ul className="flex flex-col gap-0.5">
                    {results.map((entry) => (
                        <SearchResultRow
                            key={`${entry.kind}:${entry.path}`}
                            entry={entry}
                            onOpenDirectory={onOpenDirectory}
                        />
                    ))}
                </ul>
            </div>
        </section>
    );
}

function SearchResultRow({entry, onOpenDirectory}: { entry: IndexedEntry; onOpenDirectory: (path: string) => void }) {
    const Icon = getKindIcon(entry.kind);
    const parent = getParentPath(entry.path);
    const isOpenable = entry.kind === "Directory";

    return (
        <li>
            <button
                type="button"
                onClick={() => {
                    if (isOpenable) {
                        onOpenDirectory(entry.path);
                    } else if (parent) {
                        onOpenDirectory(parent);
                    }
                }}
                title={isOpenable ? `Open ${entry.path}` : `Open containing folder: ${parent}`}
                className="group grid w-full grid-cols-[2rem_minmax(0,1fr)_auto] items-center gap-3 rounded-md border border-transparent px-2 py-2 text-left transition-colors hover:border-line-soft hover:bg-panel-soft"
            >
                <span className={iconClass(entry.kind)}>
                    <Icon className="h-4 w-4"/>
                </span>
                <span className="min-w-0">
                    <span className="block truncate text-sm font-medium text-text-main">{entry.name}</span>
                    <span className="block truncate text-xs text-text-faint">{entry.path}</span>
                </span>
                <span className="hidden text-[11px] uppercase tracking-wide text-text-faint group-hover:inline">
                    {isOpenable ? "Open folder" : "Reveal"}
                </span>
            </button>
        </li>
    );
}

function SearchHint({query}: { query: string }) {
    const isTyping = query.trim().length > 0 && query.trim().length < 2;

    return (
        <CenteredState
            icon={<Search className="h-5 w-5"/>}
            title={isTyping ? "Keep typing" : "Search your files"}
            subtitle={
                isTyping
                    ? "Enter at least 2 characters to search the index."
                    : "Start typing in the sidebar search to query indexed locations."
            }
        />
    );
}

interface CenteredStateProps {
    icon: ReactNode;
    title: string;
    subtitle?: string;
    tone?: "default" | "danger";
}

function CenteredState({icon, title, subtitle, tone = "default"}: CenteredStateProps) {
    const badgeClass =
        tone === "danger"
            ? "border-danger-line bg-danger-soft text-danger-text"
            : "border-line-soft bg-panel-bg text-accent-strong";

    return (
        <section className="flex h-full min-h-0 items-center justify-center px-8 text-center">
            <div className="flex max-w-md flex-col items-center gap-4">
                <span className={`flex h-12 w-12 items-center justify-center rounded-xl border ${badgeClass}`}>
                    {icon}
                </span>
                <div>
                    <h2 className="text-base font-semibold text-text-main">{title}</h2>
                    {subtitle && <p className="mt-1 text-sm text-text-muted">{subtitle}</p>}
                </div>
            </div>
        </section>
    );
}

function getKindIcon(kind: IndexedEntryKind) {
    if (kind === "Directory") return Folder;
    if (kind === "Symlink") return Link2;
    return FileIcon;
}

function iconClass(kind: IndexedEntryKind): string {
    const base = "flex h-8 w-8 shrink-0 items-center justify-center rounded-md border";
    if (kind === "Directory") return `${base} border-accent-muted bg-accent-soft text-accent-strong`;
    if (kind === "Symlink") return `${base} border-warn-line bg-warn-soft text-warn-text`;
    return `${base} border-line-soft bg-panel-bg text-text-muted`;
}

function getParentPath(path: string): string | undefined {
    const normalized = path.replace(/\\/g, "/").replace(/\/+$/, "");
    const lastSlash = normalized.lastIndexOf("/");
    if (lastSlash <= 0) return undefined;
    return normalized.slice(0, lastSlash);
}

function extractMessage(error: unknown): string {
    if (typeof error === "string") return error;
    if (error && typeof error === "object" && "message" in error) return String((error as {message: unknown}).message);
    return String(error);
}
