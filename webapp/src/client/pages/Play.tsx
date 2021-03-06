import {Button, Heading, HStack, Table, Tbody, Td, Text, Th, Thead, Tr} from "@chakra-ui/react"
import {Loading} from "@client/components/Loading"
import {PageWrapper} from "@client/components/PageWrapper"
import {useUserInfo} from "@client/hooks/useUserInfo"
import {GAMES} from "@shared/games"
import type {LobbyAPI} from "boardgame.io"
import {LobbyClient} from "boardgame.io/client"
import {SocketIO} from "boardgame.io/multiplayer"
import {Client} from "boardgame.io/react"
import React from "react"
import {useMutation, useQuery} from "react-query"

const GAME_SERVER_URL = import.meta.env.SNOWPACK_PUBLIC_GAME_SERVER_URL
const LOBBY_SERVER_URL = import.meta.env.SNOWPACK_PUBLIC_LOBBY_SERVER_URL

export const Play = () => {
    const [lobbyClient, setLobbyClient] = React.useState<LobbyClient | null>(null)
    const playerName = usePlayerName()

    React.useEffect(() => {
        setLobbyClient(new LobbyClient({server: LOBBY_SERVER_URL}))
    }, [])

    return <PageWrapper>
        {lobbyClient && playerName ? <PlayWrapped lobbyClient={lobbyClient} playerName={playerName} /> : <Loading centered />}
    </PageWrapper>
}

const generateName = () => `Anonymous${Math.floor(Math.random() * 100000)}`

const usePlayerName = () => {
    const userInfo = useUserInfo()
    const [playerName, setPlayerName] = React.useState<string | null>(null)

    React.useEffect(() => {
        if (userInfo.user) {
            sessionStorage.setItem("bgio-name", JSON.stringify({name: userInfo.user.display_name, isAnon: false}))
            setPlayerName(userInfo.user.display_name)
        } else {
            const json = sessionStorage.getItem("bgio-name")
            if (json) {
                try {
                    let {name, isAnon}: {name: string, isAnon: boolean} = JSON.parse(json)
                    if (!isAnon) {
                        throw Error()
                    }
                    setPlayerName(name)
                } catch {
                    const name = generateName()
                    sessionStorage.setItem("bgio-name", JSON.stringify({name, isAnon: true}))
                    setPlayerName(name)
                }
            } else {
                const name = generateName()
                sessionStorage.setItem("bgio-name", JSON.stringify({name, isAnon: true}))
                setPlayerName(name)
            }
        }
    }, [userInfo])

    return playerName
}

const useRefState = <T extends any>(initialValue: T): [T, React.Dispatch<React.SetStateAction<T>>, React.MutableRefObject<T>] => {
    const ref = React.useRef<T>(initialValue)
    const [state, setState] = React.useState<T>(ref.current)

    React.useEffect(() => {
        ref.current = state
    }, [state])

    return [state, setState, ref]
}

const PlayWrapped = ({lobbyClient, playerName}: {lobbyClient: LobbyClient, playerName: string}) => {
    const matchesQuery = useQuery("bgio-matches", async () => {
        const games = await lobbyClient.listGames()
        let matches: LobbyAPI.Match[] = []
        for (const game of games) {
            matches = matches.concat((await lobbyClient.listMatches(game)).matches)
        }
        return matches
    }, {
        refetchInterval: 1000,
        refetchOnMount: "always",
        refetchOnWindowFocus: "always"
    })
    const [matchInfo, setMatchInfo, matchInfoRef] = useRefState<{matchID: string, gameName: string, playerID?: string, credentials?: string, client: ReturnType<typeof Client>} | null>(null)
    const match = matchesQuery.data?.find(m => m.matchID === matchInfo?.matchID) || null

    React.useEffect(() => {
        const onUnmountOrUnload = async () => {
            const currentMatchInfo = matchInfoRef.current
            if (currentMatchInfo && currentMatchInfo.credentials && currentMatchInfo.playerID) {
                await lobbyClient.leaveMatch(currentMatchInfo.gameName, currentMatchInfo.matchID, {
                    playerID: currentMatchInfo.playerID,
                    credentials: currentMatchInfo.credentials
                })
            }
        }

        window.addEventListener("beforeunload", onUnmountOrUnload)

        return () => {
            window.removeEventListener("beforeunload", onUnmountOrUnload)
            onUnmountOrUnload()
        }
    }, [lobbyClient])

    React.useEffect(() => {
        if (matchInfo && matchInfo.credentials && matchInfo.playerID) {
            // Match is joined as a player, not a spectator.
            lobbyClient.updatePlayer(
                matchInfo.gameName,
                matchInfo.matchID,
                {
                    credentials: matchInfo.credentials,
                    playerID: matchInfo.playerID,
                    newName: playerName
                }
            )
        }
    }, [playerName])

    const joinMatchMutation = useMutation<{credentials: string, playerID: string}, unknown, {match: LobbyAPI.Match, playerName: string}, unknown>(async ({match, playerName}) => {
        const playerID = match.players.find(p => !p.name)?.id.toString()
        if (!playerID) {
            throw Error("no seat available in this match")
        }
        const {playerCredentials} = await lobbyClient.joinMatch(match.gameName, match.matchID, {
            playerName,
            playerID
        })
        return {credentials: playerCredentials, playerID: playerID}
    }, {
        onSuccess: ({credentials, playerID}, {match}) => setMatchInfo({
            matchID: match.matchID,
            playerID,
            credentials,
            gameName: match.gameName,
            client: Client({
                ...GAMES[match.gameName],
                multiplayer: SocketIO({server: GAME_SERVER_URL})
            })
        })
    })

    const createMatchMutation = useMutation<{matchID: string}, unknown, {gameName: string, numPlayers: number}, unknown>(async ({gameName, numPlayers}) => {
        const {matchID} = await lobbyClient.createMatch(gameName, {
            numPlayers,
        })
        return {matchID}
    }, {
        onSuccess: () => matchesQuery.refetch()
    })

    const leaveMatchMutation = useMutation<void, unknown, {match: LobbyAPI.Match, playerID: string, credentials: string}, unknown>(async ({match, playerID, credentials}) => {
        await lobbyClient.leaveMatch(match.gameName, match.matchID, {
            playerID,
            credentials
        })
    }, {
        onSuccess: () => setMatchInfo(null)
    })

    if (matchInfo && match) {
        const freeSeats = match.players.filter(p => !p.name).length
        return <>
            <HStack>
                <Button onClick={() => {
                    if (matchInfo.credentials && matchInfo.playerID) {
                        leaveMatchMutation.mutate({match, playerID: matchInfo.playerID, credentials: matchInfo.credentials})
                    } else {
                        setMatchInfo(null)
                    }
                }}>{matchInfo.credentials ? "Leave" : "Stop Spectating"}</Button>
            </HStack>
            {freeSeats === 0 ? <matchInfo.client
                matchID={match.matchID}
                playerID={matchInfo.playerID}
                credentials={matchInfo.credentials}
            /> : <Loading centered text={`Waiting for ${freeSeats} more player${freeSeats === 1 ? "" : "s"}...`} />}
        </>
    } else {
        // Display lobby to create/join/spectate game
        return <>
            <Heading>Play/Simulate</Heading>
            <Heading size="lg">Create Game</Heading>
            <Button onClick={() => createMatchMutation.mutate({gameName: "round-1", numPlayers: 2})}>Create Game</Button>
            <Heading size="lg">Join Game</Heading>
            {matchesQuery.isLoading || !matchesQuery.data ? <Loading centered /> : <MatchList matches={matchesQuery.data} onJoin={match => joinMatchMutation.mutate({match, playerName})} />}
        </>
    }
}

const MatchList = ({matches, onJoin}: {matches: LobbyAPI.Match[], onJoin: (m: LobbyAPI.Match) => void}) => {
    if (matches.length === 0) {
        return <Text>No matches are in progress.</Text>
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
            {matches.map((m, i) => <Tr key={i}>
                <Td>{m.matchID || "Unknown"}</Td>
                <Td>{m.players.filter(p => p.name).length}/{m.players.length}</Td>
                <Td>
                    <Button size="xs" onClick={() => onJoin(m)}>
                        Join
                    </Button>
                </Td>
            </Tr>)}
        </Tbody>
    </Table>
}
