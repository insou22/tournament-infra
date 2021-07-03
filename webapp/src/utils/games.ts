import type {QueryFunction} from "react-query";
import type {Game} from "src/api";
import {allGames} from "src/mocks/games";

export const getFilteredGamesList: QueryFunction<Game[], ["games", {username: string, hash?: string}] | ["games"]> = ({queryKey: [, filter]}) => {
    let url = ""
    if (filter) {
        url += `/user/${filter.username}`
        if (filter.hash) {
            url += `/binary/${filter.hash}`
        }
    }
    url += "/games"

    //const res = api.get(url)

    return allGames // res.data
}