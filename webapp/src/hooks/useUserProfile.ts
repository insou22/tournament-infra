import type {AxiosError} from "axios";
import {QueryFunction, useQuery} from "react-query";
import {api, UserProfile} from "src/api";
import {dontRetryOn404} from "src/utils/api";
import * as marcchee from "src/mocks/marcchee"

const getUserProfile: QueryFunction<UserProfile, ["userProfile", string]> = async ({queryKey: [, username]}) => {
    return marcchee.userProfile

    const response = await api.get<UserProfile>(`/user/${username}`)
    return response.data
}

export const useUserProfile = (username: string) => useQuery<unknown, AxiosError, UserProfile, ["userProfile", string]>(["userProfile", username], getUserProfile, {
    retry: dontRetryOn404,
})