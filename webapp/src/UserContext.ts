import type {SetStateAction} from "react";
import React from "react";

export interface User {
    username: string,
    display_name: string,
    current_elo: number | null
}

export const UserContext = React.createContext<{
    user: User | null,
    setUser: React.Dispatch<SetStateAction<User | null>>
}>({user: null, setUser: _ => {}})