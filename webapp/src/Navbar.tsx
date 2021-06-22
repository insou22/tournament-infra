import {Button, IconButton} from "@chakra-ui/button"
import {Box, Heading, HStack, Link, VStack} from "@chakra-ui/layout"
import React from "react"
import {NavLink, useHistory} from "react-router-dom"
import {UserContext} from "./UserContext"
import {logout} from "./utils"
import {LockIcon} from "@chakra-ui/icons"

export const Navbar = () => {
    const {user, setUser} = React.useContext(UserContext)
    const history = useHistory();

    const logoutOnClick = React.useCallback(async () => {
        logout()
        setUser(null)
    }, [setUser])
    const navigateToLogin = React.useCallback(() => {
        history.push("/login")
    }, [history])

    return <HStack justifyContent="space-between" my={4} alignItems="center">
        <NavLink to="/">
            <Link display="flex" alignItems="center">
                <img src="https://i.ytimg.com/vi/bDByGe7FgEQ/hqdefault.jpg" height="50" width="50" />
                <Heading size="md" ml={2}>MarcComp</Heading>
            </Link>
        </NavLink>
        <Box fontWeight="bold">
            <NavLink to="/rankings" activeStyle={{color: "grey"}}>Rankings</NavLink>
        </Box>
        {user ? <HStack>
            <VStack alignItems="flex-end">
                <span>{user.display_name}</span>
                <span>{user.username}</span>
            </VStack>
            <IconButton aria-label="logout" onClick={logoutOnClick} icon={<LockIcon />}/>
        </HStack> : <Button onClick={navigateToLogin}>Login</Button>}
    </HStack>
}