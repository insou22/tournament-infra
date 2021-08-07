import {ChevronDownIcon} from "@chakra-ui/icons"
import {Badge, Button, Code, Collapse, Heading, HStack, IconButton, Tab, Table, TabList, TabPanel, TabPanels, Tabs, Tbody, Td, Text, Th, Thead, Tooltip, Tr, useBoolean} from "@chakra-ui/react"
import React from "react"
import {QueryFunction, useQuery} from "react-query"
import {api, Game, Player, Stream, Turn} from "@client/api"
import {BinaryListItem} from "@client/components/BinaryListItem"
import {ButtonLink} from "@client/components/ButtonLink"
import {Loading} from "@client/components/Loading"
import {VStackPageWrapper} from "@client/components/VStackPageWrapper"
import {useBinary} from "@client/hooks/useBinary"
import {marcVsHamish} from "@client/mocks/games"
import {dontRetryOn404} from "@client/utils/api"
import {resultProps} from "@client/utils/results"
import {formatTimestamp} from "@client/utils/time"

const getGame: QueryFunction<Game, ["game", string]> = async ({queryKey: [, id]}) => {
    //return marcVsHamish
    return (await api.get(`/game/${id}`)).data
}

export const GamePage = ({id}: {id: string}) => {
    const [tabIndex, setTabIndex] = React.useState(0)
    const [expanded, setExpanded] = React.useState<number | null>(null)

    const gameQuery = useQuery(["game", id], getGame, {
        retry: dontRetryOn404
    })

    if (gameQuery.isLoading || !gameQuery.data) {
        return <Loading />
    }

    const times = formatTimestamp(gameQuery.data.created_at)

    return <VStackPageWrapper>
        {/* TODO: Add breadcrumbs. A single game is the only circumstance where we need the user/binary you navigated from for breadcrumbs, so location state should be used here to determine that. */}
        <HStack w="100%" justifyContent="space-between">
            <Heading>Game {id}</Heading>
            <Tooltip hasArrow label={times.localised}><Text fontSize="xl">{times.relative}</Text></Tooltip>
        </HStack>
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
                {gameQuery.data.turns.map((t, i) => <React.Fragment key={i}>
                    <Tr>
                        <Td border="none">{i + 1}</Td>
                        <Td border="none">{t.username}</Td>
                        <Td border="none">{t.action}</Td>
                        <Td border="none">{t.run_time_ms}ms</Td>
                        <Td border="none" display="flex" justifyContent="flex-end">
                            {t.streams && <IconButton size="xs" variant="ghost" aria-label="expand row" icon={<ChevronDownIcon />} onClick={() => setExpanded(p => p === i ? null : i)} />}
                        </Td>
                    </Tr>
                    <Tr height={0}>
                        <Td colSpan={5} p={0}>
                            {t.streams && <Collapse in={expanded === i} animateOpacity>
                                <StreamTabs tabIndex={tabIndex} onChange={i => setTabIndex(i)} streams={t.streams} />
                            </Collapse>}
                        </Td>
                    </Tr>
                </React.Fragment>)}
            </Tbody>
        </Table>
    </VStackPageWrapper>
}

const PlayerListItem = ({player}: {player: Player}) => {
    const [show, setShow] = useBoolean()

    const binaryQuery = useBinary(player.username, player.binary_hash)

    return <>
        <HStack>
            <ButtonLink href={`/user/${player.username}`}>
                {player.display_name} ({player.rating_before_game})
            </ButtonLink>
            <Badge variant="solid" colorScheme={resultProps[player.result].color}>
                {resultProps[player.result].label}
            </Badge>
            <Badge colorScheme={resultProps[player.result].color}>
                {player.rating_change > 0 && "+"}
                {player.rating_change}
            </Badge>
            <Button size="xs" onClick={setShow.toggle}>
                {show ? "Hide" : "Show"} Binary Details
            </Button>
        </HStack>
        <Collapse in={show} animateOpacity style={{width: "100%"}}>
            {binaryQuery.data ? <BinaryListItem binary={binaryQuery.data} username={player.username} /> : <Loading />}
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