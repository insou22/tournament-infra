import {Breadcrumb, BreadcrumbItem, Button, Heading, HStack, Text, Tooltip} from "@chakra-ui/react"
import type {Game} from "@client/api"
import {BinaryStatsSummary} from "@client/components/BinaryStatSummary"
import {BinaryStatusAlert} from "@client/components/BinaryStatusAlert"
import {BreadcrumbLink} from "@client/components/BreadcrumbLink"
import {GameList} from "@client/components/GameList"
import {Loading} from "@client/components/Loading"
import {PageWrapper} from "@client/components/PageWrapper"
import {useBinary} from "@client/hooks/useBinary"
import {useUserProfile} from "@client/hooks/useUserProfile"
import {dontRetryOn404} from "@client/utils/api"
import {getFilteredGamesList} from "@client/utils/games"
import {formatTimestamp} from "@client/utils/time"
import type {AxiosError} from "axios"
import React from "react"
import {useInfiniteQuery, useQuery} from "react-query"

export const BinaryPage = ({username, hash}: {username: string, hash: string}) => {
    const profileQuery = useUserProfile(username)
    const binaryQuery = useBinary(username, hash)

    if (binaryQuery.isLoading || !binaryQuery.data) {
        return <Loading />
    }

    const times = formatTimestamp(binaryQuery.data.created_at)

    let compileAlert = null

    if (binaryQuery.data.compile_result !== "success") {
        compileAlert = <BinaryStatusAlert result={binaryQuery.data.compile_result} />
    } else {
        compileAlert = <>
            {binaryQuery.data.compile_time_ms && <HStack>
                <Text fontWeight="bold">Compile Time:</Text>
                <Text>{binaryQuery.data.compile_time_ms}ms</Text>
            </HStack>}
            <BinaryStatsSummary stats={binaryQuery.data.stats_summary} />
            <Heading size="lg">Latest Games</Heading>
            <BinaryGameList username={username} hash={hash} />
        </>
    }

    return <PageWrapper>
        <Breadcrumb>
            <BreadcrumbItem>
                <BreadcrumbLink href={`/user/${username}`}>
                    {profileQuery.data?.display_name}
                </BreadcrumbLink>
            </BreadcrumbItem>
            <BreadcrumbItem>
                <BreadcrumbLink href={`/user/${username}/binaries`}>
                    Binaries
                </BreadcrumbLink>
            </BreadcrumbItem>
            <BreadcrumbItem isCurrentPage>
                <BreadcrumbLink href={`/user/${username}/binary/${hash}`}>
                    {hash}
                </BreadcrumbLink>
            </BreadcrumbItem>
        </Breadcrumb>

        <HStack w="100%" justifyContent="space-between">
            <Heading>Binary {hash}</Heading>
            <Tooltip hasArrow label={times.localised}><Text fontSize="xl">Created {times.relative}</Text></Tooltip>
        </HStack>

        {compileAlert}
    </PageWrapper>
}

const BinaryGameList = ({username, hash}: {username: string, hash: string}) => {
    const gamesQuery = useInfiniteQuery(["games", {username, hash}], getFilteredGamesList, {
        retry: dontRetryOn404,
        getNextPageParam: p => p.next_cursor
    })

    if (gamesQuery.isLoading || !gamesQuery.data) {
        return <Loading />
    }

    return <>
        <GameList games={gamesQuery.data.pages.flatMap(p => p.items)} username={username} />
        {gamesQuery.hasNextPage && <Button variant="link" onClick={() => gamesQuery.fetchNextPage()} isLoading={gamesQuery.isFetchingNextPage}>Load More...</Button>}
    </>
}