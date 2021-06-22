import {api} from "./api"
import type {User} from "./UserContext"

export const logout = async () => {
    await api.post("/logout")
}

export const login = async (username: string, password: string): Promise<User> => {
    const response = await api.post<User>("/login", {zid: username, password})
    console.log(response.data)
    return response.data
}