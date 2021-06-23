import {Box, Container, Heading} from '@chakra-ui/layout';
import React from 'react';
import './App.css';
import {Navbar} from './Navbar';
import {
    Switch,
    Route,
    Redirect
} from "react-router-dom";
import {User, UserContext} from './UserContext';
import {Rankings, Login, Spec, Faqs, About, Profile} from './pages';

interface AppProps {}

function App({}: AppProps) {
    const [user, setUser] = React.useState<User | null>(null)

    return <UserContext.Provider value={{user, setUser}}>
        <Box w="100%" height="100%">
            <Container maxW="container.xl">
                <Navbar />
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
                        <About/>
                    </Route>
                    <Route path="/profile" exact>
                        {user ? <Profile username={user.username}/> : <Redirect to="/login" />}
                    </Route>
                    <Route path="/users/:username" exact render={({match: {params: {username}}}) => <Profile username={username}/>} />
                </Switch>
            </Container>
        </Box>
    </UserContext.Provider>
}

export default App;
