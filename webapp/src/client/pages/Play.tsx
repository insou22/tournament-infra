import {Heading, Text} from "@chakra-ui/layout"
import React from "react"
import {VStackPageWrapper} from "@client/components/VStackPageWrapper"
import {LobbyClient} from "boardgame.io/client";
import {BoardProps, Client} from "boardgame.io/react"
import type {LobbyAPI, PlayerID} from "boardgame.io";
import {Loading} from "@client/components/Loading";
import {Button, FormControl, FormLabel, Input, Table, Tbody, Td, Th, Thead, Tr} from "@chakra-ui/react";
import {GAMES} from "@shared/games";
import {useMutation, useQuery} from "react-query";
import type {ClientOpts} from "boardgame.io/dist/types/src/client/client";
import {SocketIO} from "boardgame.io/multiplayer"

const lobbyClient = new LobbyClient({server: 'http://localhost:8081'});

interface GameDetails {
    matchID: string,
    playerID?: string,
    credentials?: string,
    game: string
}

export const Play = () => {
    const [gameDetails, setGameDetails] = React.useState<GameDetails | null>(null)
    const [runningMatch, setRunningMatch] = React.useState<{isRunning: true, GameClient: ReturnType<typeof Client>} | {isRunning: false, GameClient: null}>({isRunning: false, GameClient: null})

    React.useEffect(() => {
        if (runningMatch.isRunning) {
            setRunningMatch({isRunning: false, GameClient: null})
        }
        if (gameDetails) {
            const {game, board} = GAMES[gameDetails.game]
            setRunningMatch({
                isRunning: true,
                GameClient: Client({
                    ...gameDetails,
                    game,
                    board,
                    multiplayer: SocketIO({server: "http://localhost:8081"}),
                })
            })
        }
    }, [gameDetails])

    if (gameDetails) {
        if (runningMatch.isRunning) {
            return <VStackPageWrapper>
                <runningMatch.GameClient matchID={gameDetails.matchID} playerID={gameDetails.playerID} credentials={gameDetails.credentials} />
            </VStackPageWrapper>
        } else {
            return <Loading />
        }
    } else {
        return <Lobby setGameDetails={setGameDetails} />
    }
}

const Lobby = ({setGameDetails}: {setGameDetails: React.Dispatch<React.SetStateAction<GameDetails | null>>}) => {
    const createGameMutation = useMutation<GameDetails, unknown, {game: string, playerName: string}, unknown>(async ({game, playerName}) => {
        const {matchID} = await lobbyClient.createMatch(game, {numPlayers: 2, setupData: {host: "0"}})
        return {
            credentials: (await lobbyClient.joinMatch(game, matchID, {
                playerID: "0",
                playerName
            })).playerCredentials,
            playerID: "0",
            game,
            matchID
        }
    }, {
        onSuccess: r => setGameDetails(r)
    })

    return <VStackPageWrapper>
        <Heading>Play/Simulate</Heading>
        <Heading size="lg">Create Game</Heading>
        <FormControl>
            <FormLabel>Player Name</FormLabel>
            <Input placeholder="Player Name" />
        </FormControl>
        <Button isLoading={createGameMutation.isLoading} onClick={() => createGameMutation.mutate({game: "round-1", playerName: "test"})}>Create Game</Button>
        <Heading size="lg">Join Game</Heading>
        <MatchList setGameDetails={setGameDetails} />
    </VStackPageWrapper>
}

const MatchList = ({setGameDetails}: {setGameDetails: React.Dispatch<React.SetStateAction<GameDetails | null>>}) => {
    const matchesQuery = useQuery<unknown, unknown, LobbyAPI.MatchList["matches"], ["bgio-games", {game: string}]>(["bgio-games", {game: "round-1"}], async ({queryKey: [, {game}]}) => {
        return (await lobbyClient.listMatches(game, {isGameover: false})).matches
    }, {
        staleTime: 2000
    })

    const joinGameMutation = useMutation<GameDetails, Error, {match: LobbyAPI.Match, playerName: string}, unknown>(async ({match, playerName}) => {
        const playerID = match.players.find(p => !p.name)?.id
        if (playerID === undefined) {
            throw Error("No seat available in this game.")
        }
        return {
            credentials: (await lobbyClient.joinMatch(match.gameName, match.matchID, {
                playerID: playerID.toString(),
                playerName
            })).playerCredentials,
            playerID: playerID.toString(),
            game: match.gameName,
            matchID: match.matchID
        }
    }, {
        onSuccess: r => setGameDetails(r)
    })

    if (matchesQuery.isLoading || !matchesQuery.data) {
        return <Loading />
    } else if (!matchesQuery.data.length) {
        return <Text>No games are running.</Text>
    }

    return <Table>
        <Thead>
            <Tr>
                <Th>Host</Th>
                <Th>Players</Th>
                <Th></Th>
            </Tr>
        </Thead>
        <Tbody>
            {matchesQuery.data.map(m => <Tr key={m.matchID}>
                <Td>{m.players.find(p => `${p.id}` === m.setupData?.host)?.name || "Unknown"}</Td>
                <Td>{m.players.filter(p => p.name).length}/{m.players.length}</Td>
                <Td>
                    <Button size="xs" onClick={() => joinGameMutation.mutate({match: m, playerName: "test"})} isLoading={joinGameMutation.isLoading}>
                        Join
                    </Button>
                    <Button size="xs" onClick={() => setGameDetails({game: m.gameName, matchID: m.matchID})} isLoading={joinGameMutation.isLoading}>
                        Spectate
                    </Button>
                </Td>
            </Tr>)}
        </Tbody>
    </Table>
}