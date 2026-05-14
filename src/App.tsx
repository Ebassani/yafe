import reactLogo from "./assets/react.svg";
import "./App.css";
import {FileInfo, listDirectory, listUserDirectories, UserDir} from "./directory";
import {useEffect, useState} from "react";
import { Sidebar } from "./components/Sidebar";

function App() {
    const [userDirectories, setUserDirectories] = useState<UserDir[]>([]);
    const [error, setError] = useState<string | undefined>(undefined);
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
                        <h1>Welcome to Tauri + React</h1>

                        {error && (
                            <span>{error}</span>
                        )}

                        <div className="row">
                            <a href="https://vite.dev" target="_blank">
                                <img src="/vite.svg" className="logo vite" alt="Vite logo"/>
                            </a>
                            <a href="https://tauri.app" target="_blank">
                                <img src="/tauri.svg" className="logo tauri" alt="Tauri logo"/>
                            </a>
                            <a href="https://react.dev" target="_blank">
                                <img src={reactLogo} className="logo react" alt="React logo"/>
                            </a>
                        </div>
                        <p>Click on the Tauri, Vite, and React logos to learn more.</p>
                    </div>
                </div>
            </div>
        </main>
    );
}

export default App;
