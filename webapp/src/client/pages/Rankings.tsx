import {Heading} from "@chakra-ui/layout"
import {Box, HStack, Table, Tbody, Td, Th, Thead, Tr} from "@chakra-ui/react"
import {api, Ranking} from "@client/api"
import {ButtonLink} from "@client/components/ButtonLink"
import {Loading} from "@client/components/Loading"
import {VStackPageWrapper} from "@client/components/VStackPageWrapper"
import {getRankingStyles} from "@client/utils/stats"
import type {AxiosError} from "axios"
import React from "react"
import {QueryFunction, useQuery} from "react-query"


const getRankings: QueryFunction<Ranking[], ["rankings"]> = async () => {
    return (await api.get("/rankings")).data
    // return [
    //     {
    //         username: "chicken",
    //         display_name: "Chicken",
    //         rating: 9001,
    //         win_loss: Infinity
    //     },
    //     {
    //         username: "marcchee",
    //         display_name: "Marc Chee",
    //         rating: 4200,
    //         win_loss: 1
    //     },
    //     {
    //         username: "hamishwhc",
    //         display_name: "HamishWHC",
    //         rating: 1337,
    //         win_loss: 6 / 9
    //     },
    //     {
    //         username: "evil-izzy",
    //         display_name: "Evil Izzy",
    //         rating: 666,
    //         win_loss: 0.5
    //     }
    // ]
}

export const Rankings = () => {
    const rankingsQuery = useQuery<unknown, AxiosError, Ranking[], ["rankings"]>(["rankings"], getRankings)

    if (rankingsQuery.isLoading || !rankingsQuery.data) {
        return <Loading />
    }

    return <VStackPageWrapper>
        <Heading>Current Tournament Rankings</Heading>
        <Table>
            <Thead>
                <Tr>
                    <Th>Ranking</Th>
                    <Th>Player</Th>
                    <Th>Rating</Th>
                    {/* <Th>W/L</Th> */}
                </Tr>
            </Thead>
            <Tbody>
                {rankingsQuery.data.map((r, i) => <RankingItem ranking={r} n={i + 1} key={i + 1} />)}
            </Tbody>
        </Table>
    </VStackPageWrapper>
}

const RankingItem = ({ranking, n}: {ranking: Ranking, n: number}) => {
    const styles = getRankingStyles(n)
    return <Tr fontSize={n == 1 ? "2xl" : n == 2 ? "xl" : n == 3 ? "lg" : "md"} color={styles.color}>
        <Td>
            <HStack>
                <Box>{n}{styles.suffix}</Box>
                <Box>{styles.decoration}</Box>
            </HStack>
        </Td>
        <Td><ButtonLink href={`/user/${ranking.username}`} size="inherit" color={styles.color}>{ranking.display_name}</ButtonLink></Td>
        <Td>{ranking.rating}</Td>
    </Tr>
}