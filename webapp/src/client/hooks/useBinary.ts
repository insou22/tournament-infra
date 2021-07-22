import type {AxiosError} from "axios";
import {QueryFunction, useQuery} from "react-query";
import type {Binary} from "@client/api";
import {dontRetryOn404} from "@client/utils/api";
import * as marcchee from "@client/mocks/marcchee"

const getBinary: QueryFunction<Binary, ["binary", {username: string, hash: string}]> = () => {
    return marcchee.binaries[0]
}

export const useBinary = (username: string, hash: string) => useQuery<unknown, AxiosError, Binary, ["binary", {username: string, hash: string}]>(["binary", {username, hash}], getBinary, {
    retry: dontRetryOn404
})