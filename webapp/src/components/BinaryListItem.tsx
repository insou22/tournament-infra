import {VStack, Text} from "@chakra-ui/layout";
import {HStack} from "@chakra-ui/react";
import React from "react";
import type {Binary} from "src/api";
import {BinaryStatsSummary} from "./BinaryStatSummary";
import {ButtonLink} from "./ButtonLink";

export const BinaryListItem = ({binary, username}: {binary: Binary, username: string}) => <HStack w="100%" bg="gray.700" p={4} rounded="xl">
    <VStack flexGrow={1} align="flex-start">
        <ButtonLink href={`/user/${username}/binary/${binary.hash}`}>{binary.hash}</ButtonLink>
        <Text size="sm">Created at 11:32pm on 2021-06-21</Text>
    </VStack>
    <BinaryStatsSummary stats={binary.stats_summary} />
</HStack>