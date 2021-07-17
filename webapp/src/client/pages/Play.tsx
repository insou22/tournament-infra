import {Heading} from "@chakra-ui/layout"
import React from "react"
import {VStackPageWrapper} from "@client/components/VStackPageWrapper"
import {Round1Client} from "@shared/games/round-1"

export const Play = () => {
    return <VStackPageWrapper>
        <Heading>Play/Simulate</Heading>
        <Heading size="lg">Create Game</Heading>
        <Heading size="lg">Join Game</Heading>
        <Round1Client playerID="0" />
    </VStackPageWrapper>
}