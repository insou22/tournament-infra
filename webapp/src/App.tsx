import {Box, Container, Heading} from '@chakra-ui/layout';
import React from 'react';
import './App.css';
import {Navbar} from './Navbar';
import {
    Switch,
    Route,
    Redirect
} from "react-router-dom";
import {Rankings, Login, Spec, Faqs, About, Profile, Settings} from './pages';
import {LoggedInContext, useUserInfo} from './utils/auth';

interface AppProps {}

function App({}: AppProps) {
    const {data: user} = useUserInfo()

    return <Container maxW="container.xl">
        <Box pt={4} pb={10}>
            <Navbar />
        </Box>
        <Switch>
            <Route path="/" exact>
                <Heading>Home</Heading>
            </Route>
            <Route path="/login" exact>
                {user ? <Redirect to="/profile" /> : <Login />}
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
                {user ? <Profile username={user.username} /> : <Redirect to="/login" />}
            </Route>
            <Route path="/user/:username" exact render={({match: {params: {username}}}) => <Profile username={username} />} />
            <Route path="/settings" exact>
                {user ? <Settings /> : <Redirect to="/login" />}
            </Route>
        </Switch>
    </Container>
}

export default App;
