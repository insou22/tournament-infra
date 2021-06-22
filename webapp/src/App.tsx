import React from 'react';
import {Container} from 'react-bootstrap';
import './App.css';
import {TopNavbar} from './TopNavbar';

interface AppProps {}

function App({}: AppProps) {
    return <Container fluid className="bg-dark h-100">
        <Container fluid="md">
            <TopNavbar />
        </Container>
    </Container>
}

export default App;
