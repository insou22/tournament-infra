import {Heading, VStack} from "@chakra-ui/react"
import type {AxiosError} from "axios"
import React from "react"
import {useQuery} from "react-query"
import type {Game} from "src/api"
import {GameList} from "src/components/GameList"
import {Loading} from "src/components/Loading"
import {VStackPageWrapper} from "src/components/VStackPageWrapper"
import {allGames} from "src/mocks/games"
import {dontRetryOn404} from "src/utils/api"
import {getFilteredGamesList} from "src/utils/games"

export const AllGames = () => {
    const gamesQuery = useQuery<unknown, AxiosError, Game[], ["games"]>(["games"], getFilteredGamesList, {
        retry: dontRetryOn404
    })

    if (gamesQuery.isLoading || !gamesQuery.data) {
        return <Loading />
    }

    return <VStackPageWrapper>
        <Heading>Recent Games</Heading>
        <GameList games={allGames} />
    </VStackPageWrapper>
}