import {Button} from "@chakra-ui/button"
import {useBoolean} from "@chakra-ui/hooks"
import {Badge, Center, Container, Heading, HStack, Text, VStack} from "@chakra-ui/layout"
import {Spinner} from "@chakra-ui/spinner"
import type {AxiosError} from "axios"
import React from "react"
import {QueryFunction, useQuery} from "react-query"
import {BinaryListItem} from "src/components/BinaryListItem"
import {BinaryStatsSummary} from "src/components/BinaryStatSummary"
import {Loading} from "src/components/Loading"
import {StatsSummary} from "src/components/StatSummary"
import {dontRetryOn404} from "src/utils/api"
import {api, BinaryStats, TournamentStats, UserProfile} from "../api"
import {getOrdinalSuffix} from "../utils/stats"

const getUserProfile: QueryFunction<UserProfile, ["userProfile", string]> = async ({queryKey: [, username]}) => {
    return {
        username: "marcchee",
        display_name: "Marc Chee",
        current_tournament_stats_summary: {
            ranking: 4,
            wins: 247,
            losses: 194,
            draws: 34,
            elo: 1534,
            average_turn_run_time_ms: 623
        },
        current_binary: {
            hash: "2678afd65ad",
            created_at: "2021-06-23T23:12:45Z",
            stats_summary: {
                wins: 247,
                losses: 194,
                draws: 34,
                win_loss_ratio_percentage_change: 13.55,
                average_turn_run_time_ms: 623,
                average_turn_run_time_ms_percentage_change: -14.12
            }
        }
    }

    const response = await api.get<UserProfile>(`/user/${username}`)
    return response.data
}

export const Profile = ({username}: {username: string}) => {
    const [showPreviousTournaments, setShowPreviousTournaments] = useBoolean(false)

    const profileQuery = useQuery<unknown, AxiosError, UserProfile, ["userProfile", string]>(["userProfile", username], getUserProfile, {
        retry: dontRetryOn404,
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

    return <Container maxW="container.lg">
        <VStack spacing={4} alignItems="flex-start">
            <Heading size="2xl">{profileQuery.data.display_name}</Heading>
            {profileQuery.data.current_tournament_stats_summary ? <>
                <Heading size="lg">July Tournament</Heading>
                <TournamentStatsSummary stats={profileQuery.data.current_tournament_stats_summary} />
                {profileQuery.data.current_binary && <>
                    <Heading size="md">Recent Games</Heading>
                    <RecentGames />
                    <Text fontSize="sm">See More...</Text>
                    <Heading size="md">Current Binary</Heading>
                    <HStack w="100%">
                        <BinaryListItem binary={profileQuery.data.current_binary} />
                    </HStack>
                    <Text fontSize="sm">See More...</Text>
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
        </VStack>
    </Container>
}

const RecentGames = () => {
    return <VStack>
        <HStack>
            <Button variant="link">Marc Chee (9001) vs Hamish Cox (1337)</Button>
            <Badge variant="solid" colorScheme="green">Won</Badge>
            <Badge colorScheme="green">+23</Badge>
            <Text>/</Text>
            <Badge colorScheme="red">-13</Badge>
        </HStack>
    </VStack>
}

const TournamentStatsSummary = ({stats}: {stats: TournamentStats}) => {
    return <StatsSummary stats={[
        {
            label: "Ranking",
            value: `${stats.ranking}${getOrdinalSuffix(stats.ranking)}`
        },
        {
            label: "Rating",
            value: stats.elo
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
            value: (stats.wins / stats.losses).toFixed(2)
        },
        {
            label: "Draws",
            value: stats.draws
        },
        {
            label: "Average Turn Run Time",
            value: `${stats.average_turn_run_time_ms}ms`
        }
    ]} />
}