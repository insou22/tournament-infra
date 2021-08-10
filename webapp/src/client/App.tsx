import {Box, Container, Heading} from '@chakra-ui/layout'
import {AnimatePresence} from 'framer-motion'
import React from 'react'
import {Redirect, Route, Switch, useLocation} from "react-router-dom"
import {Loading} from './components/Loading'
import {Navbar} from './components/Navbar'
import {PageWrapper} from "./components/PageWrapper"
import {useUserInfo} from './hooks/useUserInfo'
import {About, AllGames, BinariesPage, BinaryPage, GamePage, Home, Login, Play, PlayerGames, Profile, Rankings, Settings, Spec} from './pages'

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
                        <Home />
                    </Route>
                    <Route path="/about" exact>
                        <About />
                    </Route>

                    <Route path="/spec" exact>
                        <Spec />
                    </Route>

                    <Route path="/play" exact>
                        <Play />
                    </Route>

                    <Route path="/login" exact>
                        {isLoading ? <Loading /> : (user ? <Redirect to={`/user/${user.username}`} /> : <Login />)}
                    </Route>
                    <Route path="/settings" exact>
                        {isLoading ? <Loading /> : (user ? <Settings /> : <Redirect to="/login" />)}
                    </Route>

                    <Route path="/user/:username" exact render={
                        ({match: {params: {username}}}) => <Profile username={username} />
                    } />
                    <Route path="/user/:username/games" exact render={
                        ({match: {params: {username}}}) => <PlayerGames username={username} />
                    } />
                    <Route path="/user/:username/binaries" exact render={
                        ({match: {params: {username}}}) => <BinariesPage username={username} />
                    } />
                    <Route path="/user/:username/binary/:hash" exact render={
                        ({match: {params: {hash, username}}}) => <BinaryPage hash={hash} username={username} />
                    } />

                    <Route path="/rankings" exact>
                        <Rankings />
                    </Route>
                    <Route path="/games" exact>
                        <AllGames />
                    </Route>

                    <Route path="/game/:id" exact render={
                        ({match: {params: {id}}}) => <GamePage id={id} />
                    } />

                    <Route>
                        <PageWrapper>
                            <Heading>404</Heading>
                            <Heading size="md">This page doesn't exist.</Heading>
                        </PageWrapper>
                    </Route>
                </Switch>
            </AnimatePresence>
        </Box>
    </Container>
}
