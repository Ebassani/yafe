import {FileInfo} from "../directory";

interface Props {
    files: FileInfo[];
}

export default function DirectoryView({files}: Props) {
    return (
        <div>
            <h1>Dir view</h1>
        </div>
    )
}