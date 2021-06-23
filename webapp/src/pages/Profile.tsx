import {Box, Center, Container, Divider, Heading, VStack} from "@chakra-ui/layout"
import {Spinner} from "@chakra-ui/spinner"
import {StatGroup, Stat, StatLabel, StatNumber, StatHelpText, StatArrow} from "@chakra-ui/stat"
import type {AxiosError} from "axios"
import React from "react"
import {QueryFunction, useQuery} from "react-query"
import {api} from "../api"

interface TournamentStats {
    wins: number,
    losses: number,
    draws: number,
    elo: number,
    average_turn_runtime_ms: number
}

interface UserProfile {
    username: string,
    display_name: string,
    tournament_stats: Record<number, TournamentStats>
}

const getUserProfile: QueryFunction<UserProfile, ["userProfile", string]> = async ({queryKey: [, username]}) => {
    return {
        username: "username",
        display_name: "User Name",
        tournament_stats: {
            1: {
                wins: 265,
                losses: 145,
                draws: 34,
                elo: 1423,
                average_turn_runtime_ms: 657
            }
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
            <Box borderColor="whiteAlpha.300" borderStyle="solid" borderWidth="1px" borderRadius={12} w="100%">
                <StatGroup p={2}>
                    <Stat>
                        <StatLabel>Rating</StatLabel>
                        <StatNumber>{profileQuery.data.tournament_stats[1].elo}</StatNumber>
                    </Stat>

                    <Stat>
                        <StatLabel>Wins</StatLabel>
                        <StatNumber>{profileQuery.data.tournament_stats[1].wins}</StatNumber>
                    </Stat>

                    <Stat>
                        <StatLabel>Losses</StatLabel>
                        <StatNumber>{profileQuery.data.tournament_stats[1].losses}</StatNumber>
                    </Stat>

                    <Stat>
                        <StatLabel>Draws</StatLabel>
                        <StatNumber>{profileQuery.data.tournament_stats[1].draws}</StatNumber>
                    </Stat>
                </StatGroup>
            </Box>

            <Heading size="md">Binaries</Heading>
            <Heading size="lg">Previous Tournaments</Heading>
            <Heading size="md">June Tournament</Heading>
            <Heading size="md">May Tournament</Heading>
            <Heading size="md">April Tournament</Heading>
        </VStack>
    </Container>
}