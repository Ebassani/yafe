import {useResizablePanel} from "../hooks/useResizablePanel";
import {UserDir} from "../directory";
import {SidebarLocationItem} from "./SidebarLocationItem";
import {Command, HomeIcon, Search, X} from "lucide-react";
import {useEffect, useRef} from "react";

interface Props {
    userDirs: UserDir[],
    selectedPath?: string;
    query: string;
    onQueryChange: (value: string) => void;
    onSelect: (path: string | undefined) => void;
}

export function Sidebar(
    {
        userDirs,
        selectedPath,
        query,
        onQueryChange,
        onSelect
    }: Props) {
    const inputRef = useRef<HTMLInputElement | null>(null);
    const panel = useResizablePanel({
        initialWidth: 264,
        minWidth: 180,
        maxWidth: 480,
        offsetLeft: 0,
    });
    const isSearching = query.trim().length > 0;

    useEffect(() => {
        const onKeyDown = (event: KeyboardEvent) => {
            const isMod = event.metaKey || event.ctrlKey;
            if (isMod && event.key.toLowerCase() === "k") {
                event.preventDefault();
                inputRef.current?.focus();
                inputRef.current?.select();
            }

            if (event.key === "Escape" && document.activeElement === inputRef.current) {
                onQueryChange("");
                inputRef.current?.blur();
            }
        };

        window.addEventListener("keydown", onKeyDown);
        return () => window.removeEventListener("keydown", onKeyDown);
    }, [onQueryChange]);

    return (
        <aside
            className="relative inset-y-0 left-0 z-auto flex w-auto shrink-0 flex-col border-r border-line-soft bg-app-rail shadow-none"
            style={{width: `min(86vw, ${panel.width}px)`}}
        >
            <div className="px-2 pb-2 pt-3">
                <div className="relative">
                    <span className="pointer-events-none absolute left-2.5 top-1/2 flex h-4 w-4 -translate-y-1/2 items-center justify-center text-text-faint">
                        <Search className="h-4 w-4"/>
                    </span>
                    <input
                        ref={inputRef}
                        type="search"
                        value={query}
                        onChange={(event) => onQueryChange(event.target.value)}
                        placeholder="Search"
                        className="h-9 w-full rounded-md border border-line-soft bg-panel-bg pl-8 pr-16 text-sm text-text-main placeholder:text-text-faint outline-none transition-colors focus:border-accent-muted"
                        spellCheck={false}
                        autoComplete="off"
                    />
                    <div className="absolute right-1.5 top-1/2 flex -translate-y-1/2 items-center">
                        {query ? (
                            <button
                                type="button"
                                onClick={() => onQueryChange("")}
                                className="flex h-6 w-6 items-center justify-center rounded-md text-text-faint transition-colors hover:bg-panel-soft hover:text-text-main"
                                title="Clear search"
                            >
                                <X className="h-3.5 w-3.5"/>
                            </button>
                        ) : (
                            <span className="flex items-center gap-1 rounded border border-line-soft bg-panel-soft px-1.5 py-0.5 text-[10px] font-medium text-text-faint">
                                <Command className="h-3 w-3"/>
                                <span>K</span>
                            </span>
                        )}
                    </div>
                </div>
            </div>

            <button
                type="button"
                onClick={() => onSelect(undefined)}
                className={[
                    "group mx-2 mb-1 flex items-center gap-2 rounded-md border px-2 py-2 text-left transition-colors",
                    !selectedPath && !isSearching
                        ? "border-accent-muted bg-accent-soft text-text-main"
                        : "border-transparent text-text-soft hover:border-line-soft hover:bg-panel-soft hover:text-text-main",
                ].join(" ")}
            >
                <span className="flex h-6 w-6 items-center justify-center text-text-faint group-hover:text-accent-strong">
                    <HomeIcon className="h-4 w-4"/>
                </span>
                <span className="text-sm font-medium">Home</span>
            </button>

            <div className="min-h-0 flex-1 overflow-y-auto px-2 py-2">
                <div className="px-2 pb-2 text-[11px] font-medium uppercase tracking-wider text-text-faint">
                    Quick access
                </div>
                <div className="flex flex-col gap-0.5">
                    {userDirs.map(directory => (
                        <SidebarLocationItem
                            key={`${directory.user_dir_type}-${directory.dir_path}`}
                            directory={directory}
                            active={selectedPath === directory.dir_path}
                            onSelect={onSelect}
                        />
                    ))}
                </div>
            </div>

            <div className="flex items-center justify-between border-t border-line-muted px-3 py-2 text-[11px] text-text-faint">
                <span>{userDirs.length} locations</span>
                <span className="flex h-1.5 w-1.5 rounded-full bg-accent-strong shadow-[0_0_6px_var(--color-accent-strong)]" title="Index ready"/>
            </div>

            <div
                {...panel.resizeHandleProps}
                className={[
                    "absolute -right-0.75 top-0 h-full w-1.5 cursor-col-resize bg-transparent transition-colors hover:bg-accent-muted block",
                    panel.dragging ? "bg-accent-muted" : "",
                ].join(" ")}
            />
        </aside>
    );
}
