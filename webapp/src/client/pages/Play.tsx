import {Button, Heading, Table, Tbody, Td, Th, Thead, Tr, Text} from "@chakra-ui/react"
import {Loading} from "@client/components/Loading"
import {VStackPageWrapper} from "@client/components/VStackPageWrapper"
import {useUserInfo} from "@client/hooks/useUserInfo"
import {GAMES} from "@shared/games"
import type {LobbyAPI} from "boardgame.io"
import {Lobby} from "boardgame.io/react"
import React from "react"
import {useMutation} from "react-query"

export const Play = () => <VStackPageWrapper>
    <Lobby
        gameComponents={Object.values(GAMES)}
        renderer={lobbyRendererWrapper}
        gameServer="http://localhost:8081"
        lobbyServer="http://localhost:8081"
    />
</VStackPageWrapper>

type LobbyRendererProps = Parameters<Required<Lobby["props"]>["renderer"]>[0]

const lobbyRendererWrapper: Required<Lobby["props"]>["renderer"] = (props) => <LobbyRenderer {...props} />

const LobbyRenderer = ({phase, matches, runningMatch, ...props}: LobbyRendererProps) => {
    const userInfo = useUserInfo()
    const [matchInfo, setMatchInfo] = React.useState<{matchID: string, playerID: string} | null>(null)
    const match = matches.find(m => m.matchID === matchInfo?.matchID) || null
    const freeSeats = match === null ? null : match?.players.filter(p => !p.name).length

    console.log(matches)

    React.useEffect(() => {
        console.log("mounted page")
        if (userInfo.user) {
            localStorage.setItem("bg-io/lobby/name", userInfo.user.display_name)
            props.handleEnterLobby(userInfo.user.display_name)
        } else {
            let name = localStorage.getItem("bg-io/lobby/name")
            if (!name || !name.startsWith("Anonymous")) {
                name = `Anonymous${Math.floor(Math.random() * 100000)}`
                localStorage.setItem("bg-io/lobby/name", name)
            }
            props.handleEnterLobby(name)
        }

        const onUnmountOrUnload = () => {
            console.log("unload called")
            props.handleExitMatch()
            props.handleExitLobby()
        }

        window.addEventListener("beforeunload", onUnmountOrUnload)

        return () => {
            window.removeEventListener("beforeunload", onUnmountOrUnload)
            onUnmountOrUnload()
        }
    }, [])

    React.useEffect(() => {
        console.log("freeSeats change")
        if (match && matchInfo) {
            if (freeSeats === 0) {
                console.log("starting")
                props.handleStartMatch("round-1", {
                    matchID: match.matchID,
                    numPlayers: match.players.length,
                    playerID: matchInfo.playerID
                })
            }
        }
    }, [freeSeats])

    const joinMatchMutation = useMutation<string, unknown, LobbyAPI.Match, unknown>(async (match) => {
        const playerID = match.players.find(p => !p.name || p.name === props.playerName)!.id.toString()
        await props.handleJoinMatch(
            match.gameName,
            match.matchID,
            playerID
        );
        return playerID
    }, {
        onSuccess: (playerID, {matchID}) => setMatchInfo({matchID, playerID})
    })

    const createMatchMutation = useMutation<void, unknown, void, unknown>(async () => {
        return props.handleCreateMatch(
            "round-1",
            2
        )
    })

    const leaveMatchMutation = useMutation<void, unknown, string, unknown>(async (matchID) => {
        console.log(`Leaving: ${matchID}`)
        await props.handleLeaveMatch("round-1", matchID)
        console.log(`Finished Leaving: ${matchID}`)
        console.log(`Exiting: ${matchID}`)
        await props.handleExitMatch()
        console.log(`Finished Existing: ${matchID}`)
    }, {
        onSuccess: () => {
            setMatchInfo(null)
        }
    })

    if (phase === "enter") {
        return <Loading centered text="Reload the page." />
    } else if (phase === "list") {
        if (match) {
            return <>
                <Button onClick={() => leaveMatchMutation.mutate(match.matchID)}>Leave</Button>
                <Loading centered text={`Waiting for ${freeSeats} more player${freeSeats !== null && freeSeats > 1 ? "s" : ""}...`} />
            </>
        }
        return <>
            <Text>{props.errorMsg}</Text>
            <Heading>Play/Simulate</Heading>
            <Heading size="lg">Create Game</Heading>
            <Button onClick={() => createMatchMutation.mutate()}>Create Game</Button>
            <Heading size="lg">Join Game</Heading>
            <Table>
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
                            <Button size="xs" onClick={() => joinMatchMutation.mutate(m)}>
                                Join
                            </Button>
                        </Td>
                    </Tr>)}
                </Tbody>
            </Table>
        </>
    } else if (phase === "play") {
        if (runningMatch) {
            return <>
                <Text>Found: {matches.find(m => m.matchID === runningMatch.matchID)?.gameName}</Text>
                <Button onClick={async () => {
                    await props.handleExitMatch()
                    await leaveMatchMutation.mutate(runningMatch.matchID)
                }}>Leave</Button>
                {match?.players.map(p => <Text>{p.name}</Text>)}
                <runningMatch.app
                    matchID={runningMatch.matchID}
                    playerID={runningMatch.playerID}
                    credentials={runningMatch.credentials}
                />
            </>
        } else {
            return <Loading centered text={`Connecting...`} />
        }
    }

    return <Loading centered text={`Connecting...? You really shouldn't be seeing this message...`} />
}

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