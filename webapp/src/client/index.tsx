import {ChakraProvider, ColorModeScript} from "@chakra-ui/react"
import React from 'react'
import ReactDOM from 'react-dom'
import {QueryClient, QueryClientProvider} from "react-query"
import {BrowserRouter as Router} from "react-router-dom"
import {App} from './App'
import './index.css'
import theme from './theme'
import {CheckUserInfoContextProvider} from "./utils/auth"

const queryClient = new QueryClient()

ReactDOM.render(
    <>
        <ColorModeScript initialColorMode={theme.config.initialColorMode} />
        <React.StrictMode>
            <QueryClientProvider client={queryClient}>
                <CheckUserInfoContextProvider>
                    <ChakraProvider theme={theme}>
                        <Router>
                            <App />
                        </Router>
                    </ChakraProvider>
                </CheckUserInfoContextProvider>
            </QueryClientProvider>
        </React.StrictMode>
    </>,
    document.getElementById('root')
)

// Hot Module Replacement (HMR) - Remove this snippet to remove HMR.
// Learn more: https://snowpack.dev/concepts/hot-module-replacement
if (import.meta.hot) {
    import.meta.hot.accept()
}
