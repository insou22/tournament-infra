import {Container, Heading, Text} from "@chakra-ui/layout"
import {Divider} from "@chakra-ui/react"
import {BinaryListItem} from "@client/components/BinaryListItem"
import {ButtonLink} from "@client/components/ButtonLink"
import {GameList} from "@client/components/GameList"
import {Loading} from "@client/components/Loading"
import {StatsSummary} from "@client/components/StatSummary"
import {PageWrapper} from "@client/components/PageWrapper"
import {useUserProfile} from "@client/hooks/useUserProfile"
import {dontRetryOn404} from "@client/utils/api"
import {getFilteredGamesList} from "@client/utils/games"
import React from "react"
import {useQuery} from "react-query"
import type {TournamentStats} from "../api"
import {getRankingStyles} from "../utils/stats"

export const Profile = ({username}: {username: string}) => {
    // const [showPreviousTournaments, setShowPreviousTournaments] = useBoolean(false)
    const profileQuery = useUserProfile(username)

    const gamesQuery = useQuery(["games", {username, perPage: 10}], getFilteredGamesList, {
        retry: dontRetryOn404,
        enabled: !!(profileQuery.data)
    })

    if (profileQuery.isError && profileQuery.error) {
        if (profileQuery.error?.response?.status === 404) {
            return <Container centerContent>
                <Heading>
                    This user doesn't exist.
                </Heading>
            </Container>
        } else {
            return <>An error occurred.</>
        }
    }

    if (profileQuery.isLoading || !profileQuery.data) {
        return <Loading />
    }

    return <PageWrapper>
        <Heading>{profileQuery.data.display_name}</Heading>
        {profileQuery.data.current_tournament_stats_summary ? <>
            <Heading size="lg">July Tournament</Heading>
            <TournamentStatsSummary stats={profileQuery.data.current_tournament_stats_summary} />
            {profileQuery.data.current_binary && <>
                <Heading size="md">Latest Games</Heading>
                {gamesQuery.data ? <GameList games={gamesQuery.data.items} username={username} /> : <Loading />}
                <ButtonLink href={`/user/${username}/games`} size="sm">See More...</ButtonLink>
                <Divider />
                <Heading size="md">Current Binary</Heading>
                <BinaryListItem binary={profileQuery.data.current_binary} username={username} />
                <ButtonLink href={`/user/${username}/binaries`} size="sm">See More...</ButtonLink>
            </>}
        </> : <Text>This user is not part of the current tournament.</Text>}

        {/* <Heading size="lg">
            Previous Tournaments
            <Button size="xs" ms={3} onClick={setShowPreviousTournaments.toggle}>
                {showPreviousTournaments ? "Hide" : "Show"}
            </Button>
        </Heading> */}
        {/* <Heading size="md">June Tournament</Heading>
        <Heading size="md">May Tournament</Heading>
        <Heading size="md">April Tournament</Heading> */}
    </PageWrapper>
}

const TournamentStatsSummary = ({stats}: {stats: TournamentStats}) => {
    return <StatsSummary stats={[
        {
            label: "Ranking",
            value: `${stats.ranking}${getRankingStyles(stats.ranking).suffix}`
        },
        {
            label: "Rating",
            value: stats.rating_mu.toFixed(2),
            subvalue: `?? ${(stats.rating_sigma * 3).toFixed(2)}`
        },
        {
            label: "Wins",
            value: stats.wins
        },
        {
            label: "Losses",
            value: stats.losses
        },
        {
            label: "W/L",
            value: (stats.win_loss ?? Infinity).toFixed(2)
        },
        {
            label: "Draws",
            value: stats.draws
        },
        {
            label: "Average Turn Run Time",
            value: `${stats.average_turn_run_time_ms.toFixed(2)}ms`
        }
    ]} />
}