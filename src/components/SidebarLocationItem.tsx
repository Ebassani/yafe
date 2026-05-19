import {
    Download,
    FileText,
    Folder,
    Home,
    Image,
    Monitor,
    Video,
    type LucideIcon
} from "lucide-react";
import {UserDir, UserDirType} from "../directory";

interface Props {
    directory: UserDir;
    active: boolean;
    onSelect: (path: string | undefined) => void;
}

const userDirIcons: Record<UserDirType, LucideIcon> = {
    Home,
    Desktop: Monitor,
    Downloads: Download,
    Documents: FileText,
    Pictures: Image,
    Videos: Video,
};

export function SidebarLocationItem({directory, active, onSelect}: Props) {
    const Icon = userDirIcons[directory.user_dir_type] ?? Folder;

    return (
        <button
            type="button"
            title={directory.dir_path}
            onClick={() => onSelect(directory.dir_path)}
            className={[
                "group mb-1 grid w-full cursor-pointer grid-cols-[2rem_minmax(0,1fr)] items-center gap-2 rounded-md border px-2 py-2 text-left transition-colors",
                active
                    ? "border-accent-muted bg-accent-soft text-text-main"
                    : "border-transparent bg-transparent text-text-soft hover:border-line-soft hover:bg-panel-soft hover:text-text-main",
            ].join(" ")}
        >
            <span
                className={[
                    "flex h-6 w-6 items-center justify-center transition-colors",
                    active
                        ? ""
                        : "text-text-faint group-hover:text-accent-strong",
                ].join(" ")}
            >
                <Icon className="h-4 w-4"/>
            </span>
            <span className="min-w-0">
                <span className="block truncate text-sm font-medium leading-5">
                    {directory.user_dir_type}
                </span>
            </span>
        </button>
    );
}
