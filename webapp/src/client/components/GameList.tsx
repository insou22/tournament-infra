import {Badge, Grid, GridItem, Text, Tooltip} from "@chakra-ui/react"
import type {Game} from "@client/api"
import {resultProps} from "@client/utils/results"
import {formatTimestamp} from "@client/utils/time"
import React from "react"
import {useHistory} from "react-router-dom"

export const GameList = ({games, username}: {games: Omit<Game, "turns">[], username?: string}) => {
    if (games.length === 0) {
        return <Text>No games.</Text>
    }
    
    return <Grid rowGap={2} columnGap={2} templateColumns={`repeat(${username ? 8 : 7}, max-content)`} fontWeight="semibold">
        {games.map((g, i) => <GameListItem game={g} username={username} row={i + 1} key={i} />)}
    </Grid>
}

const GameListItem = ({game, username, row}: {game: Omit<Game, "turns">, username?: string, row: number}) => {
    const history = useHistory()
    const firstPlayer = username ? game.players.find(p => p.username === username)! : game.players[0]
    const secondPlayer = username ? game.players.find(p => p.username !== username)! : game.players[1]

    const columnOffset = Number(!!(username))

    const times = formatTimestamp(game.created_at)

    return <>
        {username && <GridItem gridRow={row} gridColumn={1}>
            <Badge variant="solid" colorScheme={resultProps[firstPlayer.result].color} w="100%" textAlign="center">{resultProps[firstPlayer.result].label}</Badge>
        </GridItem>}
        <GridItem gridRow={row} gridColumn={1 + columnOffset}>
            <Text>{firstPlayer.display_name}</Text>
        </GridItem>
        <GridItem gridRow={row} gridColumn={2 + columnOffset}>
            <Text>({firstPlayer.rating_mu_before_game})</Text>
        </GridItem>
        <GridItem gridRow={row} gridColumn={3 + columnOffset}>
            <Badge colorScheme={resultProps[firstPlayer.result].color} w="100%" textAlign="center">
                {firstPlayer.rating_mu_change > 0 && "+"}
                {firstPlayer.rating_mu_change}
            </Badge>
        </GridItem>
        <GridItem gridRow={row} gridColumn={4 + columnOffset} placeContent="center">
            <Text>vs {secondPlayer.display_name}</Text>
        </GridItem>
        <GridItem gridRow={row} gridColumn={5 + columnOffset}>
            <Text>({secondPlayer.rating_mu_before_game})</Text>
        </GridItem>
        <GridItem gridRow={row} gridColumn={6 + columnOffset}>
            <Badge colorScheme={resultProps[secondPlayer.result].color} w="100%" textAlign="center">
                {secondPlayer.rating_mu_change > 0 && "+"}
                {secondPlayer.rating_mu_change}
            </Badge>
        </GridItem>
        <GridItem gridRow={row} gridColumn={7 + columnOffset} fontWeight="normal">
            <Tooltip hasArrow label={times.localised}>{times.relative}</Tooltip>
        </GridItem>
        <GridItem gridRow={row} gridColumnStart={1} gridColumnEnd={7 + columnOffset} onClick={() => history.push(`/game/${game.id}`)} zIndex={1} cursor="pointer" />
    </>
}