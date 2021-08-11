import {Breadcrumb, BreadcrumbItem, Button, Heading, HStack, Text, VStack} from "@chakra-ui/react"
import {api, Binary, Paginated} from "@client/api"
import {BinaryListItem} from "@client/components/BinaryListItem"
import {BreadcrumbLink} from "@client/components/BreadcrumbLink"
import {Loading} from "@client/components/Loading"
import {PageWrapper} from "@client/components/PageWrapper"
import {useUserInfo} from "@client/hooks/useUserInfo"
import {useUserProfile} from "@client/hooks/useUserProfile"
import {dontRetryOn404} from "@client/utils/api"
import type {AxiosError} from "axios"
import React from "react"
import {QueryFunction, useInfiniteQuery} from "react-query"

const getBinaries: QueryFunction<Paginated<Binary>, ["binaries", string]> = async ({queryKey: [, username], pageParam: cursor}) => {
    // return marcchee.binaries
    return (await api.get(`/user/${username}/binaries`, {params: {cursor, per_page:3}})).data
}

export const BinariesPage = ({username}: {username: string}) => {
    const {user} = useUserInfo()
    const profileQuery = useUserProfile(username)
    const binariesQuery = useInfiniteQuery<Paginated<Binary>, AxiosError, Paginated<Binary>, ["binaries", string]>(["binaries", username], getBinaries, {
        retry: dontRetryOn404,
        getNextPageParam: p => p.next_cursor
    })

    return <PageWrapper>
        <Breadcrumb>
            <BreadcrumbItem>
                <BreadcrumbLink href={`/user/${username}`}>
                    {profileQuery.data?.display_name}
                </BreadcrumbLink>
            </BreadcrumbItem>
            <BreadcrumbItem isCurrentPage>
                <BreadcrumbLink href={`/user/${username}/binaries`}>
                    Binaries
                </BreadcrumbLink>
            </BreadcrumbItem>
        </Breadcrumb>
        <HStack justify="space-between" w="100%" wrap="wrap">
            <Heading>{profileQuery.data?.display_name}'s Binaries</Heading>
            {user?.username === username && <Text>Binaries that haven't been compiled or failed compilation are only visible to you.</Text>}
        </HStack>
        {binariesQuery.isLoading ? <Loading /> : binariesQuery.data && binariesQuery.data.pages.flatMap(p => p.items).length ? <VStack spacing={4} w="100%" align="flex-start">
            {binariesQuery.data.pages.flatMap(p => p.items).map((b, i) => <BinaryListItem binary={b} username={username} key={i} />)}
            {binariesQuery.hasNextPage && <Button variant="link" onClick={() => binariesQuery.fetchNextPage()} isLoading={binariesQuery.isFetchingNextPage}>Load More...</Button>}
        </VStack> : <Heading size="md">This user doesn't have any binaries for this tournament.</Heading>}
    </PageWrapper>
}
