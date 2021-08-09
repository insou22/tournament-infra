import {api, Game} from "@client/api"
import type {QueryFunction} from "react-query"

export const getFilteredGamesList: QueryFunction<Omit<Game, "turns">[], ["games", {username: string, hash?: string, perPage?: number, page?: number}] | ["games"]> = async ({queryKey: [, filter]}) => {
    let url = ""
    if (filter) {
        url += `/user/${filter.username}`
        if (filter.hash) {
            url += `/binary/${filter.hash}`
        }
    }

    url += "/games"

    let q: {per_page?: string, page?: string} = {}

    if (filter?.perPage) {
        q.per_page = filter.perPage.toString()
    }

    if (filter?.page) {
        q.page = filter.page.toString()
    }

    url += "?" + new URLSearchParams(q).toString()

    return (await api.get(url)).data
}