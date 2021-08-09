
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
                    color: "#E5FDFF" // blue
                },
                md: {
                    color: "#FCEEFF" // pink
                },
                sm: {
                    color: "#FEFDDF" // yellow
                },
                xs: {
                    color: "#FBFDFC" // off white
                }
            }
        }
    }
})

export default theme