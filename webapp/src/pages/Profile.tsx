import {Button} from "@chakra-ui/button"
import {Box, Center, Container, Heading, VStack, Text, HStack, Badge} from "@chakra-ui/layout"
import {Spinner} from "@chakra-ui/spinner"
import {StatGroup, Stat, StatLabel, StatNumber, StatHelpText, StatArrow} from "@chakra-ui/stat"
import type {AxiosError} from "axios"
import React from "react"
import {QueryFunction, useQuery} from "react-query"
import {api} from "../api"



interface Binary {
    id: number
    created_at: string,
    time_taken_ms: number,
    timed_out: boolean,

}

interface Tournament {
    id: number
    label: string
}

type StatsMixin = {stats: Stats}

interface Stats {
    wins: number
    losses: number
    draws: number
    elo: number
    average_turn_run_time_ms: number
}

interface UserProfile {
    username: string
    display_name: string
    current_tournament_stats_summary: Stats
}

const getUserProfile: QueryFunction<UserProfile, ["userProfile", string]> = async ({queryKey: [, username]}) => {
    return {
        username: "marcchee",
        display_name: "Marc Chee",
        current_tournament_stats_summary: {
            wins: 247,
            losses: 194,
            draws: 34,
            elo: 1534,
            average_turn_run_time_ms: 623
        }
    }

    const response = await api.get<UserProfile>(`/user/${username}`)
    return response.data
}

export const Profile = ({username}: {username: string}) => {
    const profileQuery = useQuery<unknown, AxiosError, UserProfile, ["userProfile", string]>(["userProfile", username], getUserProfile, {
        retry: (count, error) => error.response?.status === 404 ? false : count < 2
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
        return <Center>
            <Spinner size="xl" />
        </Center>
    }

    return <Container maxW="container.lg">
        <VStack spacing={4} alignItems="flex-start">
            <Heading size="2xl">{profileQuery.data.display_name}</Heading>
            <Heading size="lg">July Tournament</Heading>
            <TournamentStatsSummary stats={profileQuery.data.current_tournament_stats_summary} />
            <Heading size="md">Recent Games</Heading>
            <RecentGames />
            <Text fontSize="sm">See More...</Text>
            <Heading size="md">Current Binary</Heading>
            <HStack w="100%">
                <VStack flexGrow={1} align="flex-start">
                    <Button variant="link">s76d76f6</Button>
                    <Text size="sm">Created at 11:32pm on 2021-06-21</Text>
                </VStack>
                <BinaryStatsSummary stats={profileQuery.data.current_tournament_stats_summary} changes={{win_loss_ratio: 13.55, average_turn_run_time_ms: -17.55}} />
            </HStack>
            <Text fontSize="sm">See More...</Text>
            <Heading size="lg">
                Previous Tournaments
                <Button size="xs" ms={3}>Show</Button>
            </Heading>
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

const StatsSummary = ({stats}: {
    stats: {
        label: string,
        value: number | string,
        change?: number
    }[]
}) => {
    return <Box borderColor="whiteAlpha.300" borderStyle="solid" borderWidth="1px" borderRadius={12} w="100%">
        <StatGroup p={2}>
            {stats.map((stat, i) => <Stat key={i}>
                <StatLabel>{stat.label}</StatLabel>
                <StatNumber>{stat.value}</StatNumber>
                {stat.change !== undefined && <StatHelpText>
                    <StatArrow type={stat.change < 0 ? "decrease" : "increase"} />
                    {stat.change}%
                </StatHelpText>}
            </Stat>)}
        </StatGroup>
    </Box>
}

const BinaryStatsSummary = ({stats, changes}: {stats: Stats, changes: {win_loss_ratio: number, average_turn_run_time_ms: number}}) => {
    return <StatsSummary stats={[
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
            value: (stats.wins / stats.losses).toFixed(2),
            change: changes.win_loss_ratio
        },
        {
            label: "Draws",
            value: stats.draws
        },
        {
            label: "Average Turn Run Time",
            value: `${stats.average_turn_run_time_ms}ms`,
            change: changes.average_turn_run_time_ms
        }
    ]} />
}

const TournamentStatsSummary = ({stats}: {stats: Stats}) => {
    return <StatsSummary stats={[
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