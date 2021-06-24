// A single game is the only circumstance where we need the user you navigated from for breadcrumbs, so location state should be used here to determine that.

import {ChevronDownIcon} from "@chakra-ui/icons"
import {Badge, Button, Code, Collapse, Heading, HStack, IconButton, Tab, Table, TabList, TabPanel, TabPanels, Tabs, Tbody, Td, Th, Thead, Tr, useBoolean} from "@chakra-ui/react"
import React from "react"
import {QueryFunction, useQuery} from "react-query"
import type {Game, Player, PlayerResult, Stream, Turn} from "src/api"
import {BinaryListItem} from "src/components/BinaryListItem"
import {ButtonLink} from "src/components/ButtonLink"
import {Loading} from "src/components/Loading"
import {VStackPageWrapper} from "src/components/VStackPageWrapper"
import {marcVsHamish} from "src/mocks/games"
import {dontRetryOn404} from "src/utils/api"

const getGame: QueryFunction<Game, ["game", string]> = async () => {
    return marcVsHamish
}

const resultProps: Record<PlayerResult, {label: string, color: string}> = {
    won: {
        label: "Won",
        color: "green"
    },
    lost: {
        label: "Lost",
        color: "red"
    },
    drew: {
        label: "Drew",
        color: "purple"
    }
}

export const GamePage = ({id}: {id: string}) => {
    const [tabIndex, setTabIndex] = React.useState(0)
    const [expanded, setExpanded] = React.useState<number | null>(null)

    const turns: Turn[] = [
        {
            username: "HamishWHC",
            move: "A♣",
            run_time: 234,
            streams: {
                stdin: "input\ninput\ninput",
                stdout: "output\noutput\noutput",
                stderr: "error\nerror\nerror"
            }
        },
        {
            username: "Marc Chee",
            move: "2♣",
            run_time: 234,
            streams: {
                stdin: "input\ninput\ninput",
                stdout: "output\noutput\noutput",
                stderr: "error\nerror\nerror"
            }
        },
        {
            username: "Marc Chee",
            move: "5♣",
            run_time: 234,
            streams: {
                stdin: "input\ninput\ninput",
                stdout: "output\noutput\noutput",
                stderr: "error\nerror\nerror"
            }
        }
    ]

    const gameQuery = useQuery(["game", id], getGame, {
        retry: dontRetryOn404
    })

    if (gameQuery.isLoading) {
        return <Loading />
    }

    return <VStackPageWrapper>
        <Heading>Game {id}</Heading>
        <Heading size="lg">Players</Heading>
        {gameQuery.data?.players.map((p, i) => <PlayerListItem key={i} player={p} />)}
        <Heading size="lg">Turns</Heading>
        <Table size="sm">
            <Thead>
                <Tr>
                    <Th>Turn #</Th>
                    <Th>Player</Th>
                    <Th>Move</Th>
                    <Th>Run Time</Th>
                    <Th></Th>
                </Tr>
            </Thead>
            <Tbody>
                {turns.map((t, i) => <>
                    <Tr>
                        <Td border="none">{i + 1}</Td>
                        <Td border="none">{t.username}</Td>
                        <Td border="none">{t.move}</Td>
                        <Td border="none">{t.run_time}ms</Td>
                        <Td border="none" display="flex" justifyContent="flex-end"><IconButton size="xs" variant="ghost" aria-label="expand row" icon={<ChevronDownIcon />} onClick={() => setExpanded(p => p === i ? null : i)} /></Td>
                    </Tr>
                    <Tr height={0}>
                        <Td colspan={5} p={0}>
                            <Collapse in={expanded === i} animateOpacity>
                                <StreamTabs tabIndex={tabIndex} onChange={i => setTabIndex(i)} streams={t.streams} />
                            </Collapse>
                        </Td>
                    </Tr>
                </>)}
            </Tbody>
        </Table>
    </VStackPageWrapper>
}

const PlayerListItem = ({player}: {player: Player}) => {
    const [show, setShow] = useBoolean()
    return <>
        <HStack>
            <ButtonLink href="/user/username">
                {player.user_profile.display_name} ({player.elo_before_game})
            </ButtonLink>
            <Badge variant="solid" colorScheme={resultProps[player.result].color}>
                {resultProps[player.result].label}
            </Badge>
            <Badge colorScheme={resultProps[player.result].color}>
                {player.elo_change > 0 && "+"}
                {player.elo_change}
            </Badge>
            <Button size="xs" onClick={setShow.toggle}>
                {show ? "Hide" : "Show"} Binary Details
            </Button>
        </HStack>
        <Collapse in={show} animateOpacity style={{width: "100%"}}>
            <BinaryListItem binary={player.binary} />
        </Collapse>
    </>
}

const StreamTabs = ({tabIndex, onChange, streams}: {tabIndex: number, onChange: (index: number) => void, streams: Record<Stream, string>}) => {
    return <Tabs onChange={onChange} index={tabIndex} size="md">
        <TabList>
            <Tab>Standard Input</Tab>
            <Tab>Standard Output</Tab>
            <Tab>Standard Error</Tab>
        </TabList>

        <TabPanels>
            <TabPanel>
                <Code w="100%">
                    {streams.stdin.split("\n").map(c => <>{c}<br /></>)}
                </Code>
            </TabPanel>
            <TabPanel>
                <Code w="100%">
                    {streams.stdout.split("\n").map(c => <>{c}<br /></>)}
                </Code>
            </TabPanel>
            <TabPanel>
                <Code w="100%">
                    {streams.stderr.split("\n").map(c => <>{c}<br /></>)}
                </Code>
            </TabPanel>
        </TabPanels>
    </Tabs>
}