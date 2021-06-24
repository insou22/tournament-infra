import {Heading, Text} from "@chakra-ui/layout"
import {HStack, Select, Table, Tbody, Td, Th, Thead, Tr, VStack} from "@chakra-ui/react"
import React from "react"
import {ButtonLink} from "src/components/ButtonLink"
import {VStackPageWrapper} from "src/components/VStackPageWrapper"

type Sort = "ranking" | "win_loss"

export const Rankings = () => {
    const [sort, setSort] = React.useState<Sort>("ranking")

    return <VStackPageWrapper>
        <Heading>Current Tournament Rankings</Heading>
        <HStack>
            <VStack align="flex-start">
                <Text>Sort By:</Text>
                <Select value={sort} onChange={e => setSort(e.target.value as Sort)}>
                    <option selected value="ranking">Ranking</option>
                    <option value="win_loss">W/L</option>
                </Select>
            </VStack>
        </HStack>
        <Table>
            <Thead>
                <Tr>
                    <Th>Ranking</Th>
                    <Th>Player</Th>
                    <Th>Rating</Th>
                    <Th>W/L</Th>
                </Tr>
            </Thead>
            <Tbody>
                <Tr>
                    <Td>1st</Td>
                    <Td>Chicken</Td>
                    <Td>9001</Td>
                    <Td>100</Td>
                </Tr>
                <Tr>
                    <Td>2nd</Td>
                    <Td><ButtonLink href={`/user/marcchee`}>Marc Chee</ButtonLink></Td>
                    <Td>2400</Td>
                    <Td>3.4</Td>
                </Tr>
                <Tr>
                    <Td>3rd</Td>
                    <Td>Evil Izzy</Td>
                    <Td>2400</Td>
                    <Td>1.2</Td>
                </Tr>
            </Tbody>
        </Table>
    </VStackPageWrapper>
}