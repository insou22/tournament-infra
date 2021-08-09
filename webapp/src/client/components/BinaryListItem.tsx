import {Text, VStack} from "@chakra-ui/layout"
import {HStack} from "@chakra-ui/react"
import type {Binary} from "@client/api"
import {formatTimestamp} from "@client/utils/time"
import React from "react"
import {BinaryStatsSummary} from "./BinaryStatSummary"
import {BinaryStatusAlert} from "./BinaryStatusAlert"
import {ButtonLink} from "./ButtonLink"

export const BinaryListItem = ({binary, username}: {binary: Binary, username: string}) => <HStack w="100%" bg="gray.700" p={4} rounded="xl">
    <VStack flexGrow={1} align="flex-start">
        <ButtonLink href={`/user/${username}/binary/${binary.hash}`}>{binary.hash}</ButtonLink>
        <Text size="sm">Created at {formatTimestamp(binary.created_at).localised}</Text>
    </VStack>
    {binary.compile_result === "success" ? <BinaryStatsSummary stats={binary.stats_summary} /> : <BinaryStatusAlert result={binary.compile_result} />}
</HStack>