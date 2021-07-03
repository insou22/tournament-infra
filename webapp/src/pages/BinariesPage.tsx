import {Breadcrumb, BreadcrumbItem, Heading, VStack} from "@chakra-ui/react"
import type {AxiosError} from "axios"
import React from "react"
import {QueryFunction, useQuery} from "react-query"
import type {Binary} from "src/api"
import {BinaryListItem} from "src/components/BinaryListItem"
import {BreadcrumbLink} from "src/components/BreadcrumbLink"
import {Loading} from "src/components/Loading"
import {VStackPageWrapper} from "src/components/VStackPageWrapper"
import {useUserProfile} from "src/hooks/useUserProfile"
import * as marcchee from "src/mocks/marcchee"
import {dontRetryOn404} from "src/utils/api"

const getBinaries: QueryFunction<Binary[], ["binaries", string]> = async ({queryKey: [, username]}) => {
    return marcchee.binaries
}

export const BinariesPage = ({username}: {username: string}) => {
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
        <Heading>{profileQuery.data?.display_name}'s Binaries</Heading>
        {binariesQuery.isLoading ? <Loading /> : binariesQuery.data && binariesQuery.data.length ? <VStack spacing={4} w="100%">
            {binariesQuery.data.map((b, i) => <BinaryListItem binary={b} username={username} key={i} />)}
        </VStack> : <Heading size="md">This user doesn't have any binaries for this tournament.</Heading>}
    </VStackPageWrapper>
}
