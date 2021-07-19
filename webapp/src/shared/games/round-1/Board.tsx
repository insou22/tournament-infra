import {VStack, Text} from "@chakra-ui/layout";
import {Box, Grid, GridItem, Heading, Image, Wrap} from "@chakra-ui/react";
import {CARDS} from "@shared/games/common/cards";
import type {BoardProps} from "boardgame.io/dist/types/packages/react";
import React from "react";

export const Round1Board = (props: BoardProps) => {
    return <VStack>
        <Grid templateColumns="1fr 0 1fr" columnGap={4} rowGap={8}>
            <GridItem gridColumn={1} gridRow={1} d="flex" justifyContent="center" flexDir="column" alignItems="center">
                <Heading>You</Heading>
                <Heading size="lg">Score: 5</Heading>
            </GridItem>
            <GridItem gridColumn={1} gridRow={2} d="flex" justifyContent="center">
                <Image src={CARDS["uu"]} width={40} opacity={0.5} />
            </GridItem>
            <GridItem gridColumn={1} gridRow={3}>
                <Wrap justify="center">
                    <Image src={CARDS["2C"]} />
                    <Image src={CARDS["2C"]} />
                    <Image src={CARDS["2C"]} />
                    <Image src={CARDS["2C"]} />
                    <Image src={CARDS["2C"]} />
                    <Image src={CARDS["2C"]} />
                    <Image src={CARDS["2C"]} />
                    <Image src={CARDS["2C"]} />
                    <Image src={CARDS["2C"]} />
                    <Image src={CARDS["2C"]} />
                </Wrap>
            </GridItem>
            <GridItem gridColumn={2} gridRow={2} d="flex" alignItems="center" justifyContent="center" flexDir="column">
                <Box position="absolute">
                    <Text>It's your turn.</Text>
                    <Text>Select a card!</Text>
                </Box>
            </GridItem>
            <GridItem gridColumn={3} gridRow={1} d="flex" justifyContent="center" flexDir="column" alignItems="center">
                <Heading>Other Guy</Heading>
                <Heading size="lg">Score: 5</Heading>
            </GridItem>
            <GridItem gridColumn={3} gridRow={2} d="flex" justifyContent="center">
                <Image src={CARDS["2C"]} width={40} />
            </GridItem>
            <GridItem gridColumn={3} gridRow={3}>
                <Wrap justify="center" cursor="not-allowed">
                    <Image src={CARDS["2C"]} />
                    <Image src={CARDS["2C"]} />
                    <Image src={CARDS["2C"]} />
                    <Image src={CARDS["2C"]} />
                    <Image src={CARDS["2C"]} />
                    <Image src={CARDS["2C"]} />
                    <Image src={CARDS["2C"]} />
                    <Image src={CARDS["2C"]} />
                    <Image src={CARDS["2C"]} />
                    <Image src={CARDS["2C"]} />
                </Wrap>
            </GridItem>
        </Grid>
    </VStack>
}