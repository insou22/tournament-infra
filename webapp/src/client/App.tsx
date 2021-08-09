import {Box, Container, Heading} from '@chakra-ui/layout'
import type {BoxProps} from '@chakra-ui/react'
import {AnimatePresence, motion} from 'framer-motion'
import React from 'react'
import {Redirect, Route, Switch, useLocation} from "react-router-dom"
import {Loading} from './components/Loading'
import {Navbar} from './components/Navbar'
import {useUserInfo} from './hooks/useUserInfo'
import {About, AllGames, BinariesPage, BinaryPage, Faqs, GamePage, Login, Play, PlayerGames, Profile, Rankings, Settings, Spec} from './pages'

const MotionBox = motion<BoxProps>(Box)

const pageTransitionVariants = {
    hidden: {
        opacity: 0
    },
    visible: {
        opacity: 1
    }
}

const PageTransitionWrapper: React.FC = (props) => <MotionBox initial="hidden" animate="visible" exit="hidden" variants={pageTransitionVariants}>
    {props.children}
</MotionBox>

export const App = () => {
    const {user, isLoading} = useUserInfo()
    const location = useLocation()

    return <Container maxW="container.xl">
        <Box py={4}>
            <Navbar />
        </Box>
        <Box py={6}>
            <AnimatePresence exitBeforeEnter initial={false}>
                <Switch location={location} key={location.pathname}>
                    <Route path="/" exact>
                        <PageTransitionWrapper>
                            <Heading>Home</Heading>
                        </PageTransitionWrapper>
                    </Route>
                    <Route path="/about" exact>
                        <PageTransitionWrapper>
                            <About />
                        </PageTransitionWrapper>
                    </Route>

                    <Route path="/spec" exact>
                        <PageTransitionWrapper>
                            <Spec />
                        </PageTransitionWrapper>
                    </Route>

                    <Route path="/play" exact>
                        <PageTransitionWrapper>
                            <Play />
                        </PageTransitionWrapper>
                    </Route>

                    <Route path="/login" exact>
                        <PageTransitionWrapper>
                            {isLoading ? <Loading /> : (user ? <Redirect to={`/user/${user.username}`} /> : <Login />)}
                        </PageTransitionWrapper>
                    </Route>
                    <Route path="/settings" exact>
                        <PageTransitionWrapper>
                            {isLoading ? <Loading /> : (user ? <Settings /> : <Redirect to="/login" />)}
                        </PageTransitionWrapper>
                    </Route>

                    <Route path="/user/:username" exact render={
                        ({match: {params: {username}}}) => <PageTransitionWrapper>
                            <Profile username={username} />
                        </PageTransitionWrapper>
                    } />
                    <Route path="/user/:username/games" exact render={
                        ({match: {params: {username}}}) => <PageTransitionWrapper>
                            <PlayerGames username={username} />
                        </PageTransitionWrapper>
                    } />
                    <Route path="/user/:username/binaries" exact render={
                        ({match: {params: {username}}}) => <PageTransitionWrapper>
                            <BinariesPage username={username} />
                        </PageTransitionWrapper>
                    } />
                    <Route path="/user/:username/binary/:hash" exact render={
                        ({match: {params: {hash, username}}}) => <PageTransitionWrapper>
                            <BinaryPage hash={hash} username={username} />
                        </PageTransitionWrapper>
                    } />

                    <Route path="/rankings" exact>
                        <PageTransitionWrapper>
                            <Rankings />
                        </PageTransitionWrapper>
                    </Route>
                    <Route path="/games" exact>
                        <PageTransitionWrapper>
                            <AllGames />
                        </PageTransitionWrapper>
                    </Route>

                    <Route path="/game/:id" exact render={
                        ({match: {params: {id}}}) => <PageTransitionWrapper>
                            <GamePage id={id} />
                        </PageTransitionWrapper>
                    } />
                </Switch>
            </AnimatePresence>
        </Box>
    </Container>
}
