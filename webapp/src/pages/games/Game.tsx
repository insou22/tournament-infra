// A single game is the only circumstance where we need the user you navigated from for breadcrumbs, so location state should be used here to determine that.

import {ChevronDownIcon} from "@chakra-ui/icons"
import {Badge, Box, Button, Code, Collapse, Heading, HStack, IconButton, StatDownArrow, Tab, Table, TabList, TabPanel, TabPanels, Tabs, Tbody, Td, Th, Thead, Tr, useBoolean} from "@chakra-ui/react"
import React from "react"
import {ButtonLink} from "src/components/ButtonLink"
import {VStackPageWrapper} from "src/components/VStackPageWrapper"

type Stream = "stdin" | "stdout" | "stderr"

interface Turn {
    username: string,
    move: string,
    streams: Record<Stream, string>
}

export const Game = ({id}: {id: string}) => {
    const [tabIndex, setTabIndex] = React.useState(0)
    const [expanded, setExpanded] = React.useState<number | null>(null)

    const turns: Turn[] = [
        {
            username: "HamishWHC",
            move: "A♣",
            streams: {
                stdin: "input\ninput\ninput",
                stdout: "output\noutput\noutput",
                stderr: "error\nerror\nerror"
            }
        },
        {
            username: "Marc Chee",
            move: "2♣",
            streams: {
                stdin: "input\ninput\ninput",
                stdout: "output\noutput\noutput",
                stderr: "error\nerror\nerror"
            }
        },
        {
            username: "Marc Chee",
            move: "5♣",
            streams: {
                stdin: "input\ninput\ninput",
                stdout: "output\noutput\noutput",
                stderr: "error\nerror\nerror"
            }
        }
    ]

    return <VStackPageWrapper>
        <Heading>Game {id}</Heading>
        <Heading size="lg">Players</Heading>
        <HStack>
            <ButtonLink href="/user/username">Marc Chee (9001)</ButtonLink>
            <Badge variant="solid" colorScheme="green">Won</Badge>
            <Badge colorScheme="green">+23</Badge>
        </HStack>
        <HStack>
            <ButtonLink href="/user/username">Hamish Cox (1337)</ButtonLink>
            <Badge variant="solid" colorScheme="red">Lost</Badge>
            <Badge colorScheme="red">-13</Badge>
        </HStack>
        <Heading size="lg">Turns</Heading>
        <Table size="sm">
            <Thead>
                <Tr>
                    <Th>Turn #</Th>
                    <Th>Player</Th>
                    <Th>Move</Th>
                    <Th></Th>
                </Tr>
            </Thead>
            <Tbody>
                {turns.map((t, i) => <>
                    <Tr>
                        <Td border="none">{i + 1}</Td>
                        <Td border="none">{t.username}</Td>
                        <Td border="none">{t.move}</Td>
                        <Td border="none"><IconButton size="xs" variant="ghost" aria-label="expand row" icon={<ChevronDownIcon />} onClick={() => setExpanded(p => p === i ? null : i)} /></Td>
                    </Tr>
                    <Tr height={0}>
                        <Td colspan={4} p={0}>
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

const StreamTabs = ({tabIndex, onChange, streams}: {tabIndex: number, onChange: (index: number) => void, streams: Record<Stream, string>}) => {
    return <Tabs onChange={onChange} tabIndex={tabIndex} size="md">
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