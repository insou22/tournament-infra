import {Heading} from "@chakra-ui/layout"
import {Table, Tbody, Td, Th, Thead, Tr} from "@chakra-ui/react"
import type {AxiosError} from "axios"
import React from "react"
import {QueryFunction, useQuery} from "react-query"
import {api, Ranking} from "@client/api"
import {ButtonLink} from "@client/components/ButtonLink"
import {Loading} from "@client/components/Loading"
import {VStackPageWrapper} from "@client/components/VStackPageWrapper"
import {getOrdinalSuffix} from "@client/utils/stats"


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
                {rankingsQuery.data.map((r, i) => <Tr fontSize={i == 0 ? "2xl" : i == 1 ? "xl" : i == 2 ? "lg" : "md"}>
                    <Td>{i+1}{getOrdinalSuffix(i+1)}</Td>
                    <Td><ButtonLink href={`/user/${r.username}`} size="inherit">{r.display_name}</ButtonLink></Td>
                    <Td>{r.rating}</Td>
                    {/* <Td>{r.win_loss.toFixed(2)}</Td> */}
                </Tr>)}
            </Tbody>
        </Table>
    </VStackPageWrapper>
}