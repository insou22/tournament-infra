import {Box, Container, Heading} from '@chakra-ui/layout';
import React from 'react';
import {Redirect, Route, Switch} from "react-router-dom";
import './App.css';
import {Loading} from './components/Loading';
import {Navbar} from './components/Navbar';
import {useUserInfo} from './hooks/useUserInfo';
import {About, Faqs, Login, Profile, Rankings, Settings, Spec} from './pages';
import {Binaries} from './pages/Binaries';

interface AppProps {}

function App({}: AppProps) {
    const {user, isLoading} = useUserInfo()

    return <Container maxW="container.xl">
        <Box pt={4} pb={10}>
            <Navbar />
        </Box>
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
            {/* <Route path="/user/:username/games" exact render={({match: {params: {username}}}) => <PlayerGames username={username} />} /> */}
            <Route path="/user/:username/binaries" exact render={({match: {params: {username}}}) => <Binaries username={username} />} />

            {/* <Route path="/binary/:hash" exact render={({match: {params: {hash}}}) => <Binary hash={hash} />} /> */}
            {/* <Route path="/binary/:hash/games" exact render={({match: {params: {hash}}}) => <BinaryGames hash={hash} />} /> */}

            {/* <Route path="/games" exact>
                <AllGames />
            </Route> */}
            {/* <Route path="/game/:id" exact render={({match: {params: {id}}}) => <Game id={id} />} /> */}

            <Route path="/settings" exact>
                {isLoading ? <Loading /> : (user ? <Settings /> : <Redirect to="/login" />)}
            </Route>
        </Switch>
    </Container>
}

export default App;
