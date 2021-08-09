
import {extendTheme, ThemeConfig} from "@chakra-ui/react"

const config: ThemeConfig = {
    initialColorMode: "dark",
    useSystemColorMode: false,
}

const theme = extendTheme({
    config,
    components: {
        Heading: {
            sizes: {
                // xl: {
                //     color: "#FFFFFF"
                // },
                lg: {
                    color: "#E5FDFF"
                },
                md: {
                    color: "#FCEEFF"
                },
                sm: {
                    color: "#FBFDFC"
                },
                xs: {
                    color: "#FEFDDF"
                }
            }
        }
    }
})

export default theme