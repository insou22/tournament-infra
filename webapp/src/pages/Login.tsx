import {Button} from "@chakra-ui/button"
import {useBoolean} from "@chakra-ui/hooks"
import {Input} from "@chakra-ui/input"
import {Container, Heading, VStack} from "@chakra-ui/layout"
import React from "react"
import {useHistory} from "react-router"
import {UserContext} from "../UserContext"
import {login} from "../utils"

export const Login = () => {
    const history = useHistory()
    const {setUser} = React.useContext(UserContext)
    const [username, setUsername] = React.useState("")
    const [password, setPassword] = React.useState("")
    const [loading, setLoading] = useBoolean(false)
    const [showPassword, setShowPassword] = useBoolean(false)

    const loginOnClick = React.useCallback(async () => {
        setLoading.on()
        let user = await login(username, password)
        setUser(user)
        setLoading.off()
    }, [history, username, password])

    return <Container maxW="container.sm">
        <VStack spacing={5}>
            <Heading>Login</Heading>
            <Input placeholder="zID" type="text" value={username} onChange={(e) => setUsername(e.target.value)} disabled={loading} />
            <Input placeholder="Password" type={showPassword ? "text" : "password"} value={password} onChange={(e) => setPassword(e.target.value)} disabled={loading} />
            {/* TODO: Figure out why this crashes the app. */}
            {/* <InputRightElement width="4.5rem">
                <Button h="1.75rem" size="sm" onClick={setShowPassword.toggle}>
                    {showPassword ? "Hide" : "Show"}
                </Button>
            </InputRightElement> */}
            <Button onClick={loginOnClick} disabled={loading}>Login</Button>
        </VStack>
    </Container>
}