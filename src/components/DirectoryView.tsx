import {FileInfo} from "../directory";

interface Props {
    files: FileInfo[];
}

export default function DirectoryView({files}: Props) {
    return (
        <div>
            {files.map(file => (
                <div>
                    <span>{file.file_name}</span>
                </div>
            ))}
        </div>
    )
}