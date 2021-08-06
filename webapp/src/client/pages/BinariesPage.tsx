import {Breadcrumb, BreadcrumbItem, Heading, HStack, VStack, Text} from "@chakra-ui/react"
import type {AxiosError} from "axios"
import React from "react"
import {QueryFunction, useQuery} from "react-query"
import {api, Binary} from "@client/api"
import {BinaryListItem} from "@client/components/BinaryListItem"
import {BreadcrumbLink} from "@client/components/BreadcrumbLink"
import {Loading} from "@client/components/Loading"
import {VStackPageWrapper} from "@client/components/VStackPageWrapper"
import {useUserProfile} from "@client/hooks/useUserProfile"
import * as marcchee from "@client/mocks/marcchee"
import {dontRetryOn404} from "@client/utils/api"
import {useUserInfo} from "@client/hooks/useUserInfo"

const getBinaries: QueryFunction<Binary[], ["binaries", string]> = async ({queryKey: [, username]}) => {
    // return marcchee.binaries
    return (await api.get(`/user/${username}/binaries`)).data
}

export const BinariesPage = ({username}: {username: string}) => {
    const {user} = useUserInfo()
    const profileQuery = useUserProfile(username)
    const binariesQuery = useQuery<unknown, AxiosError, Binary[], ["binaries", string]>(["binaries", username], getBinaries, {
        retry: dontRetryOn404
    })

    return <VStackPageWrapper>
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
        {binariesQuery.isLoading ? <Loading /> : binariesQuery.data && binariesQuery.data.length ? <VStack spacing={4} w="100%">
            {binariesQuery.data.map((b, i) => <BinaryListItem binary={b} username={username} key={i} />)}
        </VStack> : <Heading size="md">This user doesn't have any binaries for this tournament.</Heading>}
    </VStackPageWrapper>
}
