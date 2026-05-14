import "./App.css";
import {FileInfo, listDirectory, listUserDirectories, UserDir} from "./directory";
import {useEffect, useState} from "react";
import { Sidebar } from "./components/Sidebar";
import MainView from "./components/MainView.tsx";
import DirectoryView from "./components/DirectoryView.tsx";

function App() {
    const [userDirectories, setUserDirectories] = useState<UserDir[]>([]);
    const [_, setError] = useState<string | undefined>(undefined);
    const [sidebarOpen, setSidebarOpen] = useState<boolean>(true);
    const [selectedPath, setSelectedPath] = useState<string | undefined>(undefined);
    const [files, setFiles] = useState<FileInfo[]>([]);

    const onSelect = async (path: string | undefined) => {
        setSelectedPath(path);

        if (!path) {return}

        try {
            const dirFiles = await listDirectory(path);
            setFiles(dirFiles);
        } catch (e) {
            setError(String(e));
        }
    }

    useEffect(() => {
        (async () => {
            try {
                const dir = await listUserDirectories();
                setUserDirectories(dir);
            } catch (e) {
                setError(String(e));
            }
        })()
    }, [])

    return (
        <main className="h-screen w-full">
            <div className="flex h-full flex-col">
                <div className="relative flex min-h-0 flex-1 overflow-hidden">
                    <Sidebar userDirs={userDirectories} open={sidebarOpen}
                             onOpen={() => setSidebarOpen(true)}
                             onClose={() => setSidebarOpen(false)}
                             onSelect={onSelect}/>

                    <div className="flex min-w-0 flex-1 flex-col">
                        {!selectedPath ? (
                            <MainView />
                        ) : (
                            <DirectoryView files={files} />
                        )}
                    </div>
                </div>
            </div>
        </main>
    );
}

export default App;
