import {api, UserProfile} from "@client/api"
import {dontRetryOn404} from "@client/utils/api"
import type {AxiosError} from "axios"
import {QueryFunction, useQuery} from "react-query"

const getUserProfile: QueryFunction<UserProfile, ["userProfile", string]> = async ({queryKey: [, username]}) => {
    return (await api.get<UserProfile>(`/user/${username}`)).data
}

export const useUserProfile = (username: string) => useQuery<unknown, AxiosError, UserProfile, ["userProfile", string]>(["userProfile", username], getUserProfile, {
    retry: dontRetryOn404,
})