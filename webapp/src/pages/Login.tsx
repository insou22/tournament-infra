import {Button} from "@chakra-ui/button"
import {useBoolean} from "@chakra-ui/hooks"
import {Input} from "@chakra-ui/input"
import {Container, Heading, VStack} from "@chakra-ui/layout"
import React from "react"
import {LoggedInContext, login} from "../utils"
import {useMutation, useQueryClient} from "react-query"
import type {UserInfo} from "../utils"
import {Alert, AlertIcon, AlertTitle, AlertDescription} from "@chakra-ui/alert"

type LoginDetails = {username: string, password: string}

export const Login = () => {
    const [username, setUsername] = React.useState("")
    const [password, setPassword] = React.useState("")
    const [showPassword, setShowPassword] = useBoolean(false)
    const queryClient = useQueryClient()
    const [, setLoggedIn] = React.useContext(LoggedInContext)

    const mutation = useMutation<UserInfo, {message: string}, LoginDetails, unknown>(async ({username, password}) => {
        return await login(username, password)
    }, {
        onSuccess: user => {
            queryClient.setQueryData("currentUserInfo", user)
            setLoggedIn.on()
        }
    })

    return <Container maxW="container.sm">
        <VStack spacing={5}>
            <Heading>Login</Heading>
            {mutation.isError && mutation.error && <Alert status="error">
                <AlertIcon />
                <AlertTitle mr={2}>Login Failed: </AlertTitle>
                <AlertDescription>{mutation.error.message}</AlertDescription>
            </Alert>}
            <Input placeholder="zID" type="text" value={username} onChange={(e) => setUsername(e.target.value)} disabled={mutation.isLoading} />
            <Input placeholder="Password" type={showPassword ? "text" : "password"} value={password} onChange={(e) => setPassword(e.target.value)} disabled={mutation.isLoading} />
            {/* TODO: Figure out why this crashes the app. */}
            {/* <InputRightElement width="4.5rem">
                <Button h="1.75rem" size="sm" onClick={setShowPassword.toggle}>
                    {showPassword ? "Hide" : "Show"}
                </Button>
            </InputRightElement> */}
            <Button onClick={() => mutation.mutate({username, password})} disabled={mutation.isLoading}>Login</Button>
        </VStack>
    </Container>
}