import {Container} from "@chakra-ui/layout"
import type {AxiosError} from "axios"
import React from "react"
import {useQuery, QueryFunction} from "react-query"
import type {Binary} from "src/api"
import {BinaryListItem} from "src/components/BinaryListItem"
import {Loading} from "src/components/Loading"
import {dontRetryOn404} from "src/utils/api"

const getBinaries: QueryFunction<Binary[], ["binaries", string]> = async ({queryKey: [, username]}) => {
    return [

    ]
}

export const Binaries = ({username}: {username: string}) => {
    const binaryQuery = useQuery<unknown, AxiosError, Binary[], ["binaries", string]>(["binaries", username], getBinaries, {
        retry: dontRetryOn404
    })

    return <Container>
        {binaryQuery.isLoading || !binaryQuery.data ? <Loading /> : binaryQuery.data.map((b, i) => <BinaryListItem binary={b} key={i}/>)}
    </Container>
}