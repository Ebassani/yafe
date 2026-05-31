import {
    Download,
    FileText,
    Folder,
    Home,
    Image,
    Monitor,
    Video,
    type LucideIcon,
} from "lucide-react";
import {UserDir, UserDirType} from "../directory";

interface Props {
    userDirs: UserDir[];
    onOpen: (path: string) => void;
}

const userDirIcons: Record<UserDirType, LucideIcon> = {
    Home,
    Desktop: Monitor,
    Downloads: Download,
    Documents: FileText,
    Pictures: Image,
    Videos: Video,
};

export default function MainView({userDirs, onOpen}: Props) {
    return (
        <section className="flex h-full min-h-0 flex-col overflow-auto bg-main-bg text-text-main">
            <div className="mx-auto flex w-full max-w-4xl flex-col gap-6 px-8 py-10">
                <header>
                    <h1 className="text-xl font-semibold text-text-main">
                        Home
                    </h1>
                    <p className="mt-1 max-w-xl text-sm text-text-muted">
                        Choose a location from quick access or search indexed files from the sidebar.
                    </p>
                </header>

                <div>
                    <h2 className="mb-3 px-1 text-xs font-medium uppercase tracking-wide text-text-faint">
                        Quick access
                    </h2>
                    {userDirs.length > 0 ? (
                        <div className="grid grid-cols-1 gap-3 sm:grid-cols-2 lg:grid-cols-3">
                            {userDirs.map((dir) => (
                                <QuickAccessCard
                                    key={`${dir.user_dir_type}-${dir.dir_path}`}
                                    dir={dir}
                                    onOpen={onOpen}
                                />
                            ))}
                        </div>
                    ) : (
                        <div className="rounded-lg border border-dashed border-line-soft bg-panel-bg/40 px-4 py-10 text-center text-sm text-text-faint">
                            No user directories were detected on this system.
                        </div>
                    )}
                </div>
            </div>
        </section>
    );
}

interface QuickAccessCardProps {
    dir: UserDir;
    onOpen: (path: string) => void;
}

function QuickAccessCard({dir, onOpen}: QuickAccessCardProps) {
    const Icon = userDirIcons[dir.user_dir_type] ?? Folder;

    return (
        <button
            type="button"
            onClick={() => onOpen(dir.dir_path)}
            className="group relative flex flex-col gap-3 overflow-hidden rounded-xl border border-line-soft bg-panel-bg p-4 text-left transition-all hover:-translate-y-px hover:border-accent-muted hover:bg-panel-soft hover:shadow-[0_8px_24px_-12px_rgba(0,0,0,0.5)]"
        >
            <span className="pointer-events-none absolute -right-8 -top-8 h-24 w-24 rounded-full bg-accent/0 blur-2xl transition-all group-hover:bg-accent/10"/>

            <span className="flex h-10 w-10 items-center justify-center rounded-lg border border-accent-muted/60 bg-accent-soft text-accent-strong transition-colors">
                <Icon className="h-5 w-5"/>
            </span>

            <div className="min-w-0">
                <div className="truncate text-sm font-semibold text-text-main">
                    {dir.user_dir_type}
                </div>
                <div className="mt-0.5 truncate text-xs text-text-faint" title={dir.dir_path}>
                    {dir.dir_path}
                </div>
            </div>
        </button>
    );
}
