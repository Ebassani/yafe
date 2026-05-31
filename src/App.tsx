import "./App.css";
import {listUserDirectories, UserDir} from "./directory";
import {useEffect, useState} from "react";
import {Sidebar} from "./components/Sidebar";
import MainView from "./components/MainView.tsx";
import DirectoryView from "./components/DirectoryView.tsx";
import {SearchResults} from "./components/SearchResults";

function App() {
    const [userDirectories, setUserDirectories] = useState<UserDir[]>([]);
    const [_, setError] = useState<string | undefined>(undefined);
    const [selectedPath, setSelectedPath] = useState<string | undefined>(undefined);
    const [query, setQuery] = useState("");

    const onSelect = async (path: string | undefined) => {
        setSelectedPath(path);
        setQuery("");
    }

    const onOpenFromSearch = (path: string) => {
        setSelectedPath(path);
        setQuery("");
    };

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

    const isSearching = query.trim().length > 0;

    return (
        <main className="h-screen w-full overflow-hidden bg-main-bg text-text-main">
            <div className="relative flex h-full min-h-0 overflow-hidden bg-main-bg">
                <Sidebar
                    userDirs={userDirectories}
                    selectedPath={selectedPath}
                    query={query}
                    onQueryChange={setQuery}
                    onSelect={onSelect}
                />

                <div className="flex min-w-0 flex-1 flex-col bg-main-bg">
                    {isSearching ? (
                        <SearchResults query={query} onOpenDirectory={onOpenFromSearch}/>
                    ) : !selectedPath ? (
                        <MainView userDirs={userDirectories} onOpen={onSelect}/>
                    ) : (
                        <DirectoryView path={selectedPath}/>
                    )}
                </div>
            </div>
        </main>
    );
}

export default App;
