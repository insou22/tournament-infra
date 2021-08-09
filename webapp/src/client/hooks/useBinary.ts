import {api, Binary} from "@client/api"
import {dontRetryOn404} from "@client/utils/api"
import type {AxiosError} from "axios"
import {QueryFunction, useQuery} from "react-query"

const getBinary: QueryFunction<Binary, ["binary", {username: string, hash: string}]> = async ({queryKey: [, {username, hash}]}) => {
    return (await api.get(`/user/${username}/binary/${hash}`)).data
}

export const useBinary = (username: string, hash: string) => useQuery<unknown, AxiosError, Binary, ["binary", {username: string, hash: string}]>(["binary", {username, hash}], getBinary, {
    retry: dontRetryOn404
})