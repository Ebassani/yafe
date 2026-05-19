import {useResizablePanel} from "../hooks/useResizablePanel";
import {UserDir} from "../directory";
import {SidebarLocationItem} from "./SidebarLocationItem";

interface Props {
    userDirs: UserDir[],
    selectedPath?: string;
    onSelect: (path: string | undefined) => void;
}

export function Sidebar(
    {
        userDirs,
        selectedPath,
        onSelect
    }: Props) {
    const panel = useResizablePanel({
        initialWidth: 288,
        minWidth: 120,
        maxWidth: 600,
        offsetLeft: 0,
    });

    return (
        <>

            <aside
                className="relative inset-y-0 left-0 z-auto flex w-auto shrink-0 flex-col border-r border-line-soft bg-app-rail shadow-none"
                style={{width: `min(86vw, ${panel.width}px)`}}
            >
                <div className="min-h-0 flex-1 overflow-y-auto px-2 py-3">
                    <div className="px-2 pb-2 text-xs font-medium uppercase tracking-wide text-text-faint">
                        Quick access
                    </div>
                    {userDirs.map(directory => (
                        <SidebarLocationItem
                            key={`${directory.user_dir_type}-${directory.dir_path}`}
                            directory={directory}
                            active={selectedPath === directory.dir_path}
                            onSelect={onSelect}
                        />
                    ))}
                </div>

                <div className="border-t border-line-muted px-3 py-2 text-xs text-text-faint">
                    {userDirs.length} locations
                </div>

                <div
                    {...panel.resizeHandleProps}
                    className={[
                        "absolute -right-0.75 top-0 h-full w-1.5 cursor-col-resize bg-transparent transition-colors hover:bg-accent-muted block",
                        panel.dragging ? "bg-accent-muted" : "",
                    ].join(" ")}
                />
            </aside>
        </>
    );
}
