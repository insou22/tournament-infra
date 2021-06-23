import {Button} from "@chakra-ui/button";
import {VStack, Text} from "@chakra-ui/layout";
import React from "react";
import type {Binary} from "src/api";
import {BinaryStatsSummary} from "./BinaryStatSummary";

export const BinaryListItem = ({binary}: {binary: Binary}) => <>
    <VStack flexGrow={1} align="flex-start">
        <Button variant="link">{binary.hash}</Button>
        <Text size="sm">Created at 11:32pm on 2021-06-21</Text>
    </VStack>
    <BinaryStatsSummary stats={binary.stats_summary} />
</>