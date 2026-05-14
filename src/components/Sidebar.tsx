import {PanelLeftClose, PanelLeftOpen} from "lucide-react";
import {useResizablePanel} from "../hooks/useResizablePanel";
import {UserDir} from "../directory";

interface Props {
    userDirs: UserDir[],
    open: boolean;
    onOpen: () => void;
    onClose: () => void;
}

export function Sidebar(
    {
        userDirs,
        open,
        onOpen,
        onClose,
    }: Props) {
    const panel = useResizablePanel({
        initialWidth: 288,
        minWidth: 120,
        maxWidth: 600,
        offsetLeft: 0,
    });

    if (!open) {
        return (
            <button
                type="button"
                title="Show context"
                onClick={onOpen}
                className="hidden absolute left-3 top-3 z-30 flex h-8 w-8 cursor-pointer items-center justify-center rounded-xl border border-line-soft bg-panel-bg text-text-faint shadow-xl transition-colors hover:border-accent-muted hover:text-accent-strong"
            >
                <PanelLeftOpen className="h-4 w-4"/>
            </button>
        );
    }

    return (
        <>

            <aside
                className="inset-y-0 left-0 flex shrink-0 flex-col border-r border-line-soft bg-panel-bg relative z-auto w-auto shadow-none"
                style={{width: `min(86vw, ${panel.width}px)`}}
            >

                    <button
                        type="button"
                        title="Hide"
                        onClick={onClose}
                        className="hidden flex h-7 w-7 shrink-0 cursor-pointer items-center justify-center rounded-xl border border-transparent bg-transparent text-text-faint transition-colors hover:border-accent-muted hover:text-accent-strong"
                    >
                        <PanelLeftClose className="h-3.5 w-3.5"/>
                    </button>

                <div className="min-h-0 flex-1 flex flex-col overflow-y-auto p-3">
                    {userDirs.map(directory => (
                        <SidebarItem directory={directory} />
                    ))}
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

interface SidebarItemProps {
    directory: UserDir
}

function SidebarItem({directory}: SidebarItemProps) {
    return (
        <div className={`p-1 bg-panel-muted m-0.5`}>
            {directory.user_dir_type}
        </div>
    )
}