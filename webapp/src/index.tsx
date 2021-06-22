import React from 'react';
import ReactDOM from 'react-dom';
import App from './App';
import './index.css';

import {ChakraProvider, ColorModeScript} from "@chakra-ui/react"
import theme from './theme';

ReactDOM.render(<>
    <ColorModeScript initialColorMode={theme.config.initialColorMode} />
    <React.StrictMode>
        <ChakraProvider theme={theme}>
            <App />
        </ChakraProvider>
    </React.StrictMode>
</>,
    document.getElementById('root')
);

// Hot Module Replacement (HMR) - Remove this snippet to remove HMR.
// Learn more: https://snowpack.dev/concepts/hot-module-replacement
if (import.meta.hot) {
    import.meta.hot.accept();
}
