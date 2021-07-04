import {Breadcrumb, BreadcrumbItem, Heading} from "@chakra-ui/react"
import React from "react"
import {useQuery} from "react-query"
import {BreadcrumbLink} from "src/components/BreadcrumbLink"
import {GameList} from "src/components/GameList"
import {Loading} from "src/components/Loading"
import {VStackPageWrapper} from "src/components/VStackPageWrapper"
import {useUserProfile} from "src/hooks/useUserProfile"
import {dontRetryOn404} from "src/utils/api"
import {getFilteredGamesList} from "src/utils/games"

export const PlayerGames = ({username}: {username: string}) => {
    const profileQuery = useUserProfile(username)
    const gamesQuery = useQuery(["games", {username}], getFilteredGamesList, {
        retry: dontRetryOn404,
        enabled: !!(profileQuery.data)
    })

    if (!profileQuery.data || !gamesQuery.data) {
        return <Loading />
    }

    return <VStackPageWrapper>
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
        <GameList games={gamesQuery.data} username={username} />
    </VStackPageWrapper>
}