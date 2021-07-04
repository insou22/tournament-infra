import {Box, Container, Heading} from '@chakra-ui/layout';
import React from 'react';
import {Redirect, Route, Switch} from "react-router-dom";
import './App.css';
import {Loading} from './components/Loading';
import {Navbar} from './components/Navbar';
import {useUserInfo} from './hooks/useUserInfo';
import {About, AllGames, BinariesPage, BinaryPage, Faqs, GamePage, Login, PlayerGames, Profile, Rankings, Settings, Spec} from './pages';

interface AppProps {}

function App({}: AppProps) {
    const {user, isLoading} = useUserInfo()

    return <Container maxW="container.xl">
        <Box py={4}>
            <Navbar />
        </Box>
        <Box py={6}>
            <Switch>
                <Route path="/" exact>
                    <Heading>Home</Heading>
                </Route>
                <Route path="/login" exact>
                    {isLoading ? <Loading /> : (user ? <Redirect to="/profile" /> : <Login />)}
                </Route>
                <Route path="/rankings" exact>
                    <Rankings />
                </Route>
                <Route path="/spec" exact>
                    <Spec />
                </Route>
                <Route path="/faqs" exact>
                    <Faqs />
                </Route>
                <Route path="/about" exact>
                    <About />
                </Route>
                <Route path="/profile" exact>
                    {isLoading ? <Loading /> : (user ? <Profile username={user.username} /> : <Redirect to="/login" />)}
                </Route>
                <Route path="/user/:username" exact render={({match: {params: {username}}}) => <Profile username={username} />} />
                <Route path="/user/:username/games" exact render={({match: {params: {username}}}) => <PlayerGames username={username} />} />
                <Route path="/user/:username/binaries" exact render={({match: {params: {username}}}) => <BinariesPage username={username} />} />
                <Route path="/user/:username/binary/:hash" exact render={({match: {params: {hash, username}}}) => <BinaryPage hash={hash} username={username} />} />

                <Route path="/games" exact>
                    <AllGames />
                </Route>
                <Route path="/game/:id" exact render={({match: {params: {id}}}) => <GamePage id={id} />} />

                <Route path="/settings" exact>
                    {isLoading ? <Loading /> : (user ? <Settings /> : <Redirect to="/login" />)}
                </Route>
            </Switch>
        </Box>
    </Container>
}

export default App;
