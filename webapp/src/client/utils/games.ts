import type {QueryFunction} from "react-query";
import {api, Game} from "@client/api";
import {allGames} from "@client/mocks/games";

export const getFilteredGamesList: QueryFunction<Omit<Game, "turns">[], ["games", {username: string, hash?: string}] | ["games"]> = async ({queryKey: [, filter]}) => {
    let url = ""
    if (filter) {
        url += `/user/${filter.username}`
        if (filter.hash) {
            url += `/binary/${filter.hash}`
        }
    }
    url += "/games"

    return (await api.get(url)).data
}