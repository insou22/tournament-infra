import {Button, Heading, Table, Tbody, Td, Th, Thead, Tr, Text, HStack} from "@chakra-ui/react"
import {Loading} from "@client/components/Loading"
import {VStackPageWrapper} from "@client/components/VStackPageWrapper"
import {useUserInfo} from "@client/hooks/useUserInfo"
import {GAMES} from "@shared/games"
import type {LobbyAPI} from "boardgame.io"
import {LobbyClient} from "boardgame.io/client"
import {Client} from "boardgame.io/react"
import {SocketIO} from "boardgame.io/multiplayer"
import React from "react"
import {useMutation, useQuery} from "react-query"

export const Play = () => {
    const [lobbyClient, setLobbyClient] = React.useState<LobbyClient | null>(null)
    const playerName = usePlayerName()

    React.useEffect(() => {
        setLobbyClient(new LobbyClient({server: "http://localhost:8081"}))
    }, [])

    return lobbyClient && playerName ? <PlayWrapped lobbyClient={lobbyClient} playerName={playerName} /> : <Loading centered />
}

const usePlayerName = () => {
    const userInfo = useUserInfo()
    const [playerName, setPlayerName] = React.useState<string | null>(null)

    React.useEffect(() => {
        if (userInfo.user) {
            sessionStorage.setItem("bg-io/lobby/name", userInfo.user.display_name)
            setPlayerName(userInfo.user.display_name)
        } else {
            let name = sessionStorage.getItem("bg-io/lobby/name")
            if (!name || !name.startsWith("Anonymous")) {
                name = `Anonymous${Math.floor(Math.random() * 100000)}`
                sessionStorage.setItem("bg-io/lobby/name", name)
            }
            setPlayerName(name)
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
                multiplayer: SocketIO({server: "http://localhost:8081"})
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
        return <VStackPageWrapper>
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
        </VStackPageWrapper>
    } else {
        // Display lobby to create/join/spectate game
        return <VStackPageWrapper>
            <Heading>Play/Simulate</Heading>
            <Heading size="lg">Create Game</Heading>
            <Button onClick={() => createMatchMutation.mutate({gameName: "round-1", numPlayers: 2})}>Create Game</Button>
            <Heading size="lg">Join Game</Heading>
            {matchesQuery.isLoading || !matchesQuery.data ? <Loading centered /> : <MatchList matches={matchesQuery.data} onJoin={match => joinMatchMutation.mutate({match, playerName})} />}
        </VStackPageWrapper>
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

// type LobbyRendererProps = Parameters<Required<Lobby["props"]>["renderer"]>[0]

// const lobbyRendererWrapper: Required<Lobby["props"]>["renderer"] = (props) => <LobbyRenderer {...props} />

// const LobbyRenderer = ({phase, matches, runningMatch, ...props}: LobbyRendererProps) => {
//     const userInfo = useUserInfo()
//     const [matchInfo, setMatchInfo] = React.useState<{matchID: string, playerID: string} | null>(null)
//     const match = matches.find(m => m.matchID === matchInfo?.matchID) || null
//     const freeSeats = match === null ? null : match?.players.filter(p => !p.name).length

//     console.log(matches)

//     React.useEffect(() => {
//         console.log("mounted page")
//         if (userInfo.user) {
//             localStorage.setItem("bg-io/lobby/name", userInfo.user.display_name)
//             props.handleEnterLobby(userInfo.user.display_name)
//         } else {
//             let name = localStorage.getItem("bg-io/lobby/name")
//             if (!name || !name.startsWith("Anonymous")) {
//                 name = `Anonymous${Math.floor(Math.random() * 100000)}`
//                 localStorage.setItem("bg-io/lobby/name", name)
//             }
//             props.handleEnterLobby(name)
//         }

//         const onUnmountOrUnload = () => {
//             console.log("unload called")
//             props.handleExitMatch()
//             props.handleExitLobby()
//         }

//         window.addEventListener("beforeunload", onUnmountOrUnload)

//         return () => {
//             window.removeEventListener("beforeunload", onUnmountOrUnload)
//             onUnmountOrUnload()
//         }
//     }, [])

//     React.useEffect(() => {
//         console.log("freeSeats change")
//         if (match && matchInfo) {
//             if (freeSeats === 0) {
//                 console.log("starting")
//                 props.handleStartMatch("round-1", {
//                     matchID: match.matchID,
//                     numPlayers: match.players.length,
//                     playerID: matchInfo.playerID
//                 })
//             }
//         }
//     }, [freeSeats])

//     if (phase === "enter") {
//         return <Loading centered text="Reload the page." />
//     } else if (phase === "list") {
//         if (match) {
//             return <>
//                 <Button onClick={() => leaveMatchMutation.mutate(match.matchID)}>Leave</Button>
//                 <Loading centered text={`Waiting for ${freeSeats} more player${freeSeats !== null && freeSeats > 1 ? "s" : ""}...`} />
//             </>
//         }
//         return <>
//             <Text>{props.errorMsg}</Text>
//             <Heading>Play/Simulate</Heading>
//             <Heading size="lg">Create Game</Heading>
//             <Button onClick={() => createMatchMutation.mutate()}>Create Game</Button>
//             <Heading size="lg">Join Game</Heading>
//             <Table>
//                 <Thead>
//                     <Tr>
//                         <Th>Host</Th>
//                         <Th>Players</Th>
//                         <Th></Th>
//                     </Tr>
//                 </Thead>
//                 <Tbody>
//                     {matches.map((m, i) => <Tr key={i}>
//                         <Td>{m.matchID || "Unknown"}</Td>
//                         <Td>{m.players.filter(p => p.name).length}/{m.players.length}</Td>
//                         <Td>
//                             <Button size="xs" onClick={() => joinMatchMutation.mutate(m)}>
//                                 Join
//                             </Button>
//                         </Td>
//                     </Tr>)}
//                 </Tbody>
//             </Table>
//         </>
//     } else if (phase === "play") {
//         if (runningMatch) {
//             return <>
//                 <Text>Found: {matches.find(m => m.matchID === runningMatch.matchID)?.gameName}</Text>
//                 <Button onClick={async () => {
//                     await props.handleExitMatch()
//                     await leaveMatchMutation.mutate(runningMatch.matchID)
//                 }}>Leave</Button>
//                 {match?.players.map(p => <Text>{p.name}</Text>)}
//                 <runningMatch.app
//                     matchID={runningMatch.matchID}
//                     playerID={runningMatch.playerID}
//                     credentials={runningMatch.credentials}
//                 />
//             </>
//         } else {
//             return <Loading centered text={`Connecting...`} />
//         }
//     }

//     return <Loading centered text={`Connecting...? You really shouldn't be seeing this message...`} />
// }

/*
<div id="lobby-view" style={{ padding: 50 }}>
    <div className={this._getPhaseVisibility(LobbyPhases.ENTER)}>
        <LobbyLoginForm
        key={playerName}
        playerName={playerName}
        onEnter={this._enterLobby}
        />
    </div>

    <div className={this._getPhaseVisibility(LobbyPhases.LIST)}>
        <p>Welcome, {playerName}</p>

        <div className="phase-title" id="match-creation">
        <span>Create a match:</span>
        <LobbyCreateMatchForm
            games={gameComponents}
            createMatch={this._createMatch}
        />
        </div>
        <p className="phase-title">Join a match:</p>
        <div id="instances">
        <table>
            <tbody>
            {this.renderMatches(this.connection.matches, playerName)}
            </tbody>
        </table>
        <span className="error-msg">
            {errorMsg}
            <br />
        </span>
        </div>
        <p className="phase-title">
        Matches that become empty are automatically deleted.
        </p>
    </div>

    <div className={this._getPhaseVisibility(LobbyPhases.PLAY)}>
        {runningMatch && (
        <runningMatch.app
            matchID={runningMatch.matchID}
            playerID={runningMatch.playerID}
            credentials={runningMatch.credentials}
        />
        )}
        <div className="buttons" id="match-exit">
        <button onClick={this._exitMatch}>Exit match</button>
        </div>
    </div>

    <div className="buttons" id="lobby-exit">
        <button onClick={this._exitLobby}>Exit lobby</button>
    </div>
</div>
*/