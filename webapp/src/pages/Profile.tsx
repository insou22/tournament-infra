import {useBoolean} from "@chakra-ui/hooks"
import {Badge, Container, Heading, HStack, Text, VStack} from "@chakra-ui/layout"
import {Box, Divider, Grid, GridItem} from "@chakra-ui/react"
import React from "react"
import {useHistory} from "react-router-dom"
import {BinaryListItem} from "src/components/BinaryListItem"
import {ButtonLink} from "src/components/ButtonLink"
import {Loading} from "src/components/Loading"
import {StatsSummary} from "src/components/StatSummary"
import {VStackPageWrapper} from "src/components/VStackPageWrapper"
import {useUserProfile} from "src/hooks/useUserProfile"
import type {TournamentStats} from "../api"
import {getOrdinalSuffix} from "../utils/stats"

export const Profile = ({username}: {username: string}) => {
    // const [showPreviousTournaments, setShowPreviousTournaments] = useBoolean(false)
    const profileQuery = useUserProfile(username)
    const history = useHistory()

    if (profileQuery.isError && profileQuery.error) {
        if (profileQuery.error?.response?.status === 404) {
            return <Container centerContent>
                <Heading>
                    This user doesn't exist.
                </Heading>
            </Container>
        } else {
            return <>An error occurred.</>
        }
    }

    if (profileQuery.isLoading || !profileQuery.data) {
        return <Loading />
    }

    return <VStackPageWrapper>
        <Heading>{profileQuery.data.display_name}</Heading>
        {profileQuery.data.current_tournament_stats_summary ? <>
            <Heading size="lg">July Tournament</Heading>
            <TournamentStatsSummary stats={profileQuery.data.current_tournament_stats_summary} />
            {profileQuery.data.current_binary && <>
                <Heading size="md">Latest Games</Heading>
                <LatestGames />
                <ButtonLink href={`/user/${username}/games`} size="sm">See More...</ButtonLink>
                <Divider />
                <Heading size="md">Current Binary</Heading>
                <BinaryListItem binary={profileQuery.data.current_binary} />
                <ButtonLink href={`/user/${username}/binaries`} size="sm">See More...</ButtonLink>
            </>}
        </> : <Text>This user is not part of the current tournament.</Text>}

        {/* <Heading size="lg">
            Previous Tournaments
            <Button size="xs" ms={3} onClick={setShowPreviousTournaments.toggle}>
                {showPreviousTournaments ? "Hide" : "Show"}
            </Button>
        </Heading> */}
        {/* <Heading size="md">June Tournament</Heading>
        <Heading size="md">May Tournament</Heading>
        <Heading size="md">April Tournament</Heading> */}
    </VStackPageWrapper>
}

const LatestGames = () => {
    return <Grid rowGap={4} columnGap={2} templateColumns="repeat(2, max-content)">
        <GridItem>
            <Badge variant="solid" colorScheme="green" w="100%" textAlign="center">Won</Badge>
        </GridItem>
        <HStack>
            <Text>Marc Chee (9001)</Text>
            <Badge colorScheme="green">+23</Badge>
            <Text>vs Hamish Cox (1337)</Text>
            <Badge colorScheme="red">-13</Badge>
        </HStack>
        
        <GridItem>
            <Badge variant="solid" colorScheme="red" w="100%" textAlign="center">Lost</Badge>
        </GridItem>
        <HStack>
            <Text>Marc Chee (901)</Text>
            <Badge colorScheme="red">-13</Badge>
            <Text>vs Chicken (1234)</Text>
            <Badge colorScheme="green">+23</Badge>
        </HStack>
    </Grid>
}

const TournamentStatsSummary = ({stats}: {stats: TournamentStats}) => {
    return <StatsSummary stats={[
        {
            label: "Ranking",
            value: `${stats.ranking}${getOrdinalSuffix(stats.ranking)}`
        },
        {
            label: "Rating",
            value: stats.elo
        },
        {
            label: "Wins",
            value: stats.wins
        },
        {
            label: "Losses",
            value: stats.losses
        },
        {
            label: "W/L",
            value: (stats.wins / stats.losses).toFixed(2)
        },
        {
            label: "Draws",
            value: stats.draws
        },
        {
            label: "Average Turn Run Time",
            value: `${stats.average_turn_run_time_ms}ms`
        }
    ]} />
}