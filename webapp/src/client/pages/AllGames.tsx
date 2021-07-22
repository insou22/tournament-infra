import {Heading, VStack} from "@chakra-ui/react"
import type {AxiosError} from "axios"
import React from "react"
import {useQuery} from "react-query"
import type {Game} from "@client/api"
import {GameList} from "@client/components/GameList"
import {Loading} from "@client/components/Loading"
import {VStackPageWrapper} from "@client/components/VStackPageWrapper"
import {allGames} from "@client/mocks/games"
import {dontRetryOn404} from "@client/utils/api"
import {getFilteredGamesList} from "@client/utils/games"

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