import {useBoolean} from "@chakra-ui/hooks"
import React from "react"
import {useQuery} from "react-query"
import {api} from "../api"

//@ts-ignore
export const LoggedInContext = React.createContext<ReturnType<typeof useBoolean>>(null)

export const logout = async () => {
    await api.post("/logout")
}

export const login = async (username: string, password: string): Promise<UserInfo> => {
    const response = await api.post<UserInfo & {status: "Success"} | {message: string, status: "Failure"}>("/login", {zid: username, password})
    console.log(response.data)
    if (response.data.status === "Failure")
        throw Error(response.data.message)
    return response.data
}

export interface UserInfo {
    username: string,
    display_name: string,
    current_elo: number | null
}

export const getUserInfo = async () => {
    return (await api.get<UserInfo>("/user")).data
}

export const LoggedInContextProvider: React.FC<{}> = ({children}) => {
    const values = useBoolean(false)

    return <LoggedInContext.Provider value={values}>
        {children}
    </LoggedInContext.Provider>
}

export const useUserInfo = () => {
    const [loggedIn] = React.useContext(LoggedInContext)
    return useQuery("currentUserInfo", getUserInfo, {enabled: loggedIn})
}