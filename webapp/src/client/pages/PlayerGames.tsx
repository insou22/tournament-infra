import {Breadcrumb, BreadcrumbItem, Button, Heading} from "@chakra-ui/react"
import {BreadcrumbLink} from "@client/components/BreadcrumbLink"
import {GameList} from "@client/components/GameList"
import {Loading} from "@client/components/Loading"
import {PageWrapper} from "@client/components/PageWrapper"
import {useUserProfile} from "@client/hooks/useUserProfile"
import {dontRetryOn404} from "@client/utils/api"
import {getFilteredGamesList} from "@client/utils/games"
import React from "react"
import {useInfiniteQuery} from "react-query"

export const PlayerGames = ({username}: {username: string}) => {
    const profileQuery = useUserProfile(username)
    const gamesQuery = useInfiniteQuery(["games", {username}], getFilteredGamesList, {
        retry: dontRetryOn404,
        getNextPageParam: p => p.next_cursor,
        enabled: !!(profileQuery.data)
    })

    if (!profileQuery.data || !gamesQuery.data) {
        return <Loading />
    }

    return <PageWrapper>
        <Breadcrumb>
            <BreadcrumbItem>
                <BreadcrumbLink href={`/user/${username}`}>
                    {profileQuery.data?.display_name}
                </BreadcrumbLink>
            </BreadcrumbItem>
            <BreadcrumbItem isCurrentPage>
                <BreadcrumbLink href={`/user/${username}/games`}>
                    Games
                </BreadcrumbLink>
            </BreadcrumbItem>
        </Breadcrumb>

        <Heading>{profileQuery.data.display_name}'s Games</Heading>
        <GameList games={gamesQuery.data.pages.flatMap(p => p.items)} username={username} />
        {gamesQuery.hasNextPage && <Button variant="link" onClick={() => gamesQuery.fetchNextPage()} isLoading={gamesQuery.isFetchingNextPage}>Load More...</Button>}
    </PageWrapper>
}