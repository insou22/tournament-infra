import {Breadcrumb, BreadcrumbItem, BreadcrumbLink as ChakraBreadcrumbLink, Button, Heading, VStack} from "@chakra-ui/react"
import type {AxiosError} from "axios"
import React from "react"
import {QueryFunction, useQuery} from "react-query"
import {useHistory} from "react-router-dom"
import type {Binary} from "src/api"
import {BinaryListItem} from "src/components/BinaryListItem"
import {Loading} from "src/components/Loading"
import {VStackPageWrapper} from "src/components/VStackPageWrapper"
import {useUserProfile} from "src/hooks/useUserProfile"
import {dontRetryOn404} from "src/utils/api"

const getBinaries: QueryFunction<Binary[], ["binaries", string]> = async ({queryKey: [, username]}) => {
    return [
        {
            hash: "65d687f5fa",
            created_at: "2021-05-12T23:23:23Z",
            stats_summary: {
                wins: 123,
                losses: 234,
                win_loss_ratio_percentage_change: 23.40,
                draws: 23,
                average_turn_run_time_ms: 957,
                average_turn_run_time_ms_percentage_change: 1
            }
        },
        {
            hash: "war3drwd4w",
            created_at: "2021-05-12T23:24:23Z",
            stats_summary: {
                wins: 1234,
                losses: 243,
                win_loss_ratio_percentage_change: -10.40,
                draws: 2,
                average_turn_run_time_ms: 250,
                average_turn_run_time_ms_percentage_change: -14
            }
        }
    ]
}

const BreadcrumbLinkButton: React.FC<{href: string}> = ({href, children}) => {
    const history = useHistory()
    return <Button variant="link" onClick={() => history.push(href)}>
        {children}
    </Button>
}

const BreadcrumbLink: React.FC<{href: string}> = ({children, href}) => <ChakraBreadcrumbLink as={BreadcrumbLinkButton} href={href}>
    {children}
</ChakraBreadcrumbLink>

export const Binaries = ({username}: {username: string}) => {
    const profileQuery = useUserProfile(username)
    const binaryQuery = useQuery<unknown, AxiosError, Binary[], ["binaries", string]>(["binaries", username], getBinaries, {
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
        {binaryQuery.isLoading ? <Loading /> : binaryQuery.data && binaryQuery.data.length ? <VStack spacing={4} w="100%">
            {binaryQuery.data.map((b, i) => <BinaryListItem binary={b} key={i} />)}
        </VStack> : <Heading size="md">This user doesn't have any binaries for this tournament.</Heading>}
    </VStackPageWrapper>
}
