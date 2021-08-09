import type {QueryFunction} from "react-query";
import {api, Game} from "@client/api";
import {allGames} from "@client/mocks/games";

export const getFilteredGamesList: QueryFunction<Omit<Game, "turns">[], ["games", {username: string, hash?: string, perPage?: number, page?: number}] | ["games"]> = async ({queryKey: [, filter]}) => {
    let url = ""
    if (filter) {
        url += `/user/${filter.username}`
        if (filter.hash) {
            url += `/binary/${filter.hash}`
        }
    }

    url += "/games"

    if (filter?.perPage || filter?.page) {
        url += "?"

        if (filter.perPage) {
            url += `per_page=${filter.perPage}`
        }

        if (filter.perPage && filter.page) {
            url += "&"
        }

        if (filter.page) {
            url += `page=${filter.page}`
        }
    }

    return (await api.get(url)).data
}