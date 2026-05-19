
import {FolderSearch} from "lucide-react";

export default function MainView() {
    return (
        <section className="flex h-full min-h-0 items-center justify-center bg-main-bg px-6 text-text-main">
            <div className="flex max-w-sm flex-col items-center gap-4 text-center">
                <span className="flex h-12 w-12 items-center justify-center rounded-md border border-line-soft bg-panel-bg text-accent-strong">
                    <FolderSearch className="h-6 w-6"/>
                </span>
                <div>
                    <h1 className="text-base font-semibold text-text-main">No folder selected</h1>
                </div>
            </div>
        </section>
    )
}
