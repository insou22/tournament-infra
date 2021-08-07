
import {extendTheme, ThemeConfig} from "@chakra-ui/react"

const config: ThemeConfig = {
    initialColorMode: "dark",
    useSystemColorMode: false,
}

const theme = extendTheme({
    config,
    // components: {
    //     Heading: {
    //         sizes: {
    //             xl: {
    //                 color: "#FF0000"
    //             },
    //             lg: {
    //                 color: "#FF0000"
    //             },
    //             md: {
    //                 color: "#FF0000"
    //             },
    //             sm: {
    //                 color: "#FF0000"
    //             },
    //             xs: {
    //                 color: "#FF0000"
    //             }
    //         }
    //     }
    // }
})

export default theme