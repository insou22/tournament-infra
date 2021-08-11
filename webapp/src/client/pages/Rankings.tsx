import {Heading} from "@chakra-ui/layout"
import {Box, Button, HStack, Table, Tbody, Td, Th, Thead, Tr} from "@chakra-ui/react"
import {api, Paginated, Ranking} from "@client/api"
import {ButtonLink} from "@client/components/ButtonLink"
import {Loading} from "@client/components/Loading"
import {PageWrapper} from "@client/components/PageWrapper"
import {getRankingStyles} from "@client/utils/stats"
import type {AxiosError} from "axios"
import React from "react"
import {QueryFunction, useInfiniteQuery} from "react-query"


const getRankings: QueryFunction<Ranking[], ["rankings"]> = async ({pageParam: cursor}) => {
    return (await api.get("/rankings", {params: {cursor}})).data
}

export const Rankings = () => {
    const rankingsQuery = useInfiniteQuery<unknown, AxiosError, Paginated<Ranking>, ["rankings"]>(["rankings"], getRankings, {
        getNextPageParam: p => p.next_cursor
    })

    if (rankingsQuery.isLoading || !rankingsQuery.data) {
        return <Loading />
    }

    return <PageWrapper>
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
                {rankingsQuery.data.pages.flatMap(p => p.items).map((r, i) => <RankingItem ranking={r} n={i + 1} key={i + 1} />)}
            </Tbody>
        </Table>
        {rankingsQuery.hasNextPage && <Button variant="link" onClick={() => rankingsQuery.fetchNextPage()} isLoading={rankingsQuery.isFetchingNextPage}>Load More...</Button>}
    </PageWrapper>
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