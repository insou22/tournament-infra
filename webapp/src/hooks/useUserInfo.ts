import type {AxiosError} from "axios"
import React from "react"
import {useQuery} from "react-query"
import {api} from "src/api"
import {dontRetryOn401} from "src/utils/api"
import {CheckUserInfoContext, UserInfo} from "src/utils/auth"

const getUserInfo = async () => {
    console.log("fetching")
    return (await api.get<UserInfo>("/user")).data
}

export const useUserInfo = () => {
    const [check, setCheck] = React.useContext(CheckUserInfoContext)
    const query = useQuery<UserInfo, AxiosError, UserInfo, "currentUserInfo">("currentUserInfo", getUserInfo, {
        enabled: check,
        retry: dontRetryOn401,
        onSuccess: () => setCheck.on(),
        onError: () => setCheck.off()
    })
    return {user: check ? query.data || null : null, isLoading: check ? query.isLoading : false}
}