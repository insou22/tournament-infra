import {Box, Container, Heading} from '@chakra-ui/layout';
import React from 'react';
import './App.css';
import {Navbar} from './Navbar';
import {
    Switch,
    Route,
    Redirect
} from "react-router-dom";
import {UserContext} from './UserContext';
import {LoginPage} from './pages/login';

interface AppProps {}

function App({}: AppProps) {
    const {user} = React.useContext(UserContext)

    return <Box w="100%" height="100%">
        <Container maxW="container.xl">
            <Navbar />
            <Switch>
                <Route path="/" exact>
                    <Heading>Home</Heading>
                </Route>
                <Route path="/login" exact>
                    {user ? <Redirect to="/profile" /> : <LoginPage />}
                </Route>
                <Route path="/rankings" exact>
                    <Heading>Current Tournament Rankings</Heading>
                </Route>
                <Route path="/spec" exact>
                    <Heading>Current Tournament Spec</Heading>
                </Route>
                <Route path="/faqs" exact>
                    <Heading>FAQS</Heading>
                </Route>
                <Route path="/about" exact>
                    <Heading>About</Heading>
                </Route>
                <Route path="/profile" exact>
                    <Heading>Current User's Profile</Heading>
                </Route>
                <Route path="/users/:username" exact>
                    <Heading>Specific User's Profile</Heading>
                </Route>
            </Switch>
        </Container>
    </Box>
}

export default App;
