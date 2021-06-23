import {Button, IconButton} from "@chakra-ui/button"
import {SettingsIcon} from "@chakra-ui/icons"
import {Box, HStack, Text, VStack} from "@chakra-ui/layout"
import React from "react"
import {useHistory, useRouteMatch} from "react-router-dom"
import {useUserInfo} from "./utils"

export const Navbar = () => {
    const {data: user} = useUserInfo()
    const history = useHistory();

    return <HStack justifyContent="space-between" mt={4} mb={10} alignItems="center">
        <Box display="flex" alignItems="center" onClick={() => history.push("/")} cursor="pointer">
            <img src="https://i.ytimg.com/vi/bDByGe7FgEQ/hqdefault.jpg" height="50" width="50" />
            <Text fontSize="lg" ml={2} fontWeight="bold">MarcComp</Text>
        </Box>
        <HStack>
            <NavbarLink text="About" path="/about" exact={false} />
            <NavbarLink text="FAQs" path="/faqs" exact={false} />
            <NavbarLink text="Spec" path="/spec" exact={false} />
            <NavbarLink text="Rankings" path="/rankings" exact={false} />
        </HStack>
        {user ? <HStack spacing={2}>
            <VStack alignItems="flex-end" spacing={1}>
                <Button fontSize="md" variant="link" onClick={() => history.push("/profile")}>{user.display_name}</Button>
                <Text fontSize="xs">{user.current_elo ? `${user.current_elo}` : "Unrated"}</Text>
            </VStack>
            <NavbarIconLink aria-label="settings" path="/settings" icon={<SettingsIcon />} exact={false} />
        </HStack> : <NavbarLink text="Login" path="/login" exact={true} />}
    </HStack>
}

const NavbarLink = ({text, path, exact, ...rest}: {text: string, path: string, exact: boolean}) => {
    const history = useHistory()
    const match = useRouteMatch({path, exact})

    return <Button onClick={() => history.push(path)} variant={match ? "solid" : "ghost"} {...rest}>{text}</Button>
}

const NavbarIconLink = ({path, exact, icon, "aria-label": label, ...rest}: {"aria-label": string, path: string, exact: boolean, icon: React.ReactElement<any, string | React.JSXElementConstructor<any>>}) => {
    const history = useHistory()
    const match = useRouteMatch({path, exact})

    return <IconButton aria-label={label} onClick={() => history.push(path)} variant={match ? "solid" : "ghost"} icon={icon} {...rest}/>
}