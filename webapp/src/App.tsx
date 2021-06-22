import {Box, Container} from '@chakra-ui/layout';
import React from 'react';
import './App.css';
import {Navbar} from './Navbar';

interface AppProps {}

function App({}: AppProps) {
    return <Box w="100%" height="100%">
        <Container maxW="container.lg">
            <Navbar />
        </Container>
    </Box>
}

export default App;
