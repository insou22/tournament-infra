import React from "react"
import {useQuery} from "react-query"
import {api} from "src/api"
import {LoggedInContext, UserInfo} from "src/utils/auth"

const getUserInfo = async () => {
    return (await api.get<UserInfo>("/user")).data
}

export const useUserInfo = () => {
    const [loggedIn] = React.useContext(LoggedInContext)
    return useQuery("currentUserInfo", getUserInfo, {enabled: loggedIn})
}