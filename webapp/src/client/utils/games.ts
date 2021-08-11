import {api, Game, Paginated} from "@client/api"
import type {QueryFunction} from "react-query"

export const getFilteredGamesList: QueryFunction<Paginated<Omit<Game, "turns">>, ["games", {username?: string, hash?: string, perPage?: number, cursor?: string}] | ["games"]> = async ({queryKey: [, filter], pageParam}) => {
    let url = ""
    if (filter?.username) {
        url += `/user/${filter.username}`
        if (filter.hash) {
            url += `/binary/${filter.hash}`
        }
    }

    url += "/games"

    let q: {per_page?: string, cursor?: string} = {}

    if (filter?.perPage) {
        q.per_page = filter.perPage.toString()
    }

    if (filter?.cursor) {
        q.cursor = filter.cursor
    }

    if (pageParam) {
        q.cursor = pageParam
    }

    url += "?" + new URLSearchParams(q).toString()

    return (await api.get(url)).data
}