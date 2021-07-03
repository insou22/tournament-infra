import {Alert, AlertDescription, AlertIcon, AlertTitle, Breadcrumb, BreadcrumbItem, Heading, HStack, Text, Tooltip} from "@chakra-ui/react"
import type {AxiosError} from "axios"
import React from "react"
import {QueryFunction, useQuery} from "react-query"
import type {Binary, Game} from "src/api"
import {BreadcrumbLink} from "src/components/BreadcrumbLink"
import {VStackPageWrapper} from "src/components/VStackPageWrapper"
import {useUserProfile} from "src/hooks/useUserProfile"
import {dontRetryOn404} from "src/utils/api"
import {formatTimestamp} from "src/utils/time"
import {Loading} from "src/components/Loading"
import {GameList} from "src/components/GameList"
import {getFilteredGamesList} from "src/utils/games"
import {useBinary} from "src/hooks/useBinary"

export const BinaryPage = ({username, hash}: {username: string, hash: string}) => {
    const profileQuery = useUserProfile(username)
    const binaryQuery = useBinary(username, hash)

    if (binaryQuery.isLoading || !binaryQuery.data) {
        return <Loading />
    }

    const times = formatTimestamp(binaryQuery.data.created_at)

    let compileAlert = null;

    if (!binaryQuery.data.compile_result) {
        compileAlert = <Alert status="info">
            <AlertIcon />
            <AlertTitle mr={2}>Binary is still being compiled.</AlertTitle>
        </Alert>
    } else if (binaryQuery.data.compile_result.status === "success") {
        compileAlert = <>
            <HStack>
                <Text fontWeight="bold">Compile Time:</Text>
                <Text>{binaryQuery.data.compile_result.time_taken_ms}ms</Text>
            </HStack>
            <Heading size="lg">Latest Games</Heading>
            <BinaryGameList username={username} hash={hash} />
        </>
    } else {
        compileAlert = <Alert status="error">
            <AlertIcon />
            <AlertTitle mr={2}>Binary failed to compile!</AlertTitle>
            <AlertDescription>This binary did not compile successfully ({binaryQuery.data.compile_result.reason === "error" ? "an error occurred" : "timed out"}), so will not be used in games.</AlertDescription>
        </Alert>
    }

    return <VStackPageWrapper>
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
    </VStackPageWrapper>
}

const BinaryGameList = ({username, hash}: {username: string, hash: string}) => {
    const gamesQuery = useQuery<unknown, AxiosError, Game[], ["games", {username: string, hash: string}]>(["games", {username, hash}], getFilteredGamesList, {
        retry: dontRetryOn404
    })

    if (gamesQuery.isLoading || !gamesQuery.data) {
        return <Loading />
    }

    return <GameList games={gamesQuery.data} username={username} />
}