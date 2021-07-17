import {HStack, VStack} from "@chakra-ui/layout";
import {CARDS} from "@shared/common/cards";
import type {BoardProps} from "boardgame.io/dist/types/packages/react";
import React from "react";

export const Round1Board = (props: BoardProps) => {
    return <VStack>
        <HStack>
            <img src={CARDS["2C"]} />
        </HStack>
    </VStack>
}