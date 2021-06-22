import {Button} from "@chakra-ui/button"
import {Heading, HStack, Link} from "@chakra-ui/layout"
import React from "react"
import {UserContext} from "./UserContext"

export const Navbar = () => {
    const {user} = React.useContext(UserContext)
    return <HStack justifyContent="space-between" my={2} alignItems="center">
        <Link href="/" display="flex" alignItems="center">
            <img src="https://i.ytimg.com/vi/bDByGe7FgEQ/hqdefault.jpg" height="50" width="50"/>
            <Heading size="md" ml={2}>MarcComp</Heading>
        </Link>
        <div>
            Links here
        </div>
        {user ? <></> : <Button>Login</Button>}
    </HStack>
}