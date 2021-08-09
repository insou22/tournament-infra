import {Heading} from "@chakra-ui/react"
import {GameList} from "@client/components/GameList"
import {Loading} from "@client/components/Loading"
import {VStackPageWrapper} from "@client/components/VStackPageWrapper"
import {dontRetryOn404} from "@client/utils/api"
import {getFilteredGamesList} from "@client/utils/games"
import React from "react"
import {useQuery} from "react-query"

export const AllGames = () => {
    const gamesQuery = useQuery(["games"], getFilteredGamesList, {
        retry: dontRetryOn404
    })

    if (gamesQuery.isLoading || !gamesQuery.data) {
        return <Loading />
    }

    return <VStackPageWrapper>
        <Heading>Recent Games</Heading>
        <GameList games={gamesQuery.data} />
    </VStackPageWrapper>
}