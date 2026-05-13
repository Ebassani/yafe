import {UserDir} from "../directory";

interface Props {
    userDirs: UserDir[]
}

export function Sidebar({userDirs}: Props) {

    return (
        <aside>
            {userDirs.map(directory => (
                    <span>{directory.user_dir_type}</span>
                )
            )}
        </aside>
    )
}