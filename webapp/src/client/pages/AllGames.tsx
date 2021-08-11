import {Button, Heading} from "@chakra-ui/react"
import {GameList} from "@client/components/GameList"
import {Loading} from "@client/components/Loading"
import {PageWrapper} from "@client/components/PageWrapper"
import {dontRetryOn404} from "@client/utils/api"
import {getFilteredGamesList} from "@client/utils/games"
import React from "react"
import {useInfiniteQuery} from "react-query"

export const AllGames = () => {
    const gamesQuery = useInfiniteQuery(["games"], getFilteredGamesList, {
        retry: dontRetryOn404,
        getNextPageParam: p => p.next_cursor
    })

    if (gamesQuery.isLoading || !gamesQuery.data) {
        return <Loading />
    }

    return <PageWrapper>
        <Heading>Recent Games</Heading>
        <GameList games={gamesQuery.data.pages.flatMap(p => p.items)} />
        {gamesQuery.hasNextPage && <Button variant="link" onClick={() => gamesQuery.fetchNextPage()} isLoading={gamesQuery.isFetchingNextPage}>Load More...</Button>}
    </PageWrapper>
}