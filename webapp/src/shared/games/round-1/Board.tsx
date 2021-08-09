import {Text, VStack} from "@chakra-ui/layout"
import {Box, Grid, GridItem, Heading, Image, Wrap} from "@chakra-ui/react"
import {CardName, CARDS} from "@shared/games/common/cards"
import type {BoardProps} from "boardgame.io/dist/types/packages/react"
import React from "react"
import type {Card, State} from "./game"

const cardToCardName = (card: Card | null): CardName | null => {
    if (!card) return null
    switch (card.rank) {
        case 1:
            return `A${card.suit}`
        case 11:
            return `J${card.suit}`
        case 12:
            return `Q${card.suit}`
        case 13:
            return `K${card.suit}`
        default:
            return `${card.rank}${card.suit}` as CardName
    }
}

interface SideProps {
    playerName: string,
    score: number,
    column: number,
    playedCard: CardName | null,
    hand: Card[],
    active: boolean
    onCardClick: (i: number) => void
}

const Side = ({playerName, score, column, playedCard, hand, active, onCardClick}: SideProps) => {
    return <>
        <GridItem gridColumn={column} gridRow={1} d="flex" justifyContent="center" flexDir="column" alignItems="center">
            <Heading>{playerName}</Heading>
            <Heading size="lg">Score: {score}</Heading>
        </GridItem>
        <GridItem gridColumn={column} gridRow={2} d="flex" justifyContent="center">
            <Image src={CARDS[playedCard || "uu"]} width={40} opacity={!playedCard || playedCard === "uu" ? 0.5 : 1} />
        </GridItem>
        <GridItem gridColumn={column} gridRow={3}>
            <Wrap justify="center" cursor={active ? "inherit" : "not-allowed"}>
                {hand.map((c, i) => <Image src={CARDS[cardToCardName(c) || "uu"]} key={i} cursor={active ? "pointer" : "inherit"} onClick={active ? () => onCardClick(i) : undefined} />)}
            </Wrap>
        </GridItem>
    </>
}

export const Board = ({G, ...props}: BoardProps<State>) => {
    const onCardClick = (i: number) => {
        props.moves.playCard(i)
    }

    return <VStack w="100%">
        <Grid templateColumns="1fr 0 1fr" columnGap={4} rowGap={8} w="100%">
            <Side
                playerName={props.playerID === "0" ? "You" : props.matchData?.find(p => p.id === 0)?.name || "Unknown"}
                score={G.players["0"].score}
                column={1}
                playedCard={cardToCardName(G.players["0"].played)}
                hand={G.players["0"].hand}
                active={props.playerID === "0" && props.isActive}
                onCardClick={onCardClick}
            />
            <GridItem gridColumn={2} gridRow={2} d="flex" alignItems="center" justifyContent="center" flexDir="column">
                <Box position="absolute">
                    {
                        props.isActive ? <>
                            <Text>It's your turn.</Text>
                            <Text>Select a card!</Text>
                        </> : <>
                            <Text>Waiting for opponent's move...</Text>
                        </>
                    }
                </Box>
            </GridItem>
            <Side
                playerName={props.playerID === "1" ? "You" : props.matchData?.find(p => p.id === 1)?.name || "Unknown"}
                score={G.players["1"].score}
                column={3}
                playedCard={cardToCardName(G.players["1"].played)}
                hand={G.players["1"].hand}
                active={props.playerID === "1" && props.isActive}
                onCardClick={onCardClick}
            />
        </Grid>
    </VStack>
}