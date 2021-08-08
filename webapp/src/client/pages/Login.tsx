import {Button} from "@chakra-ui/button"
import {useBoolean} from "@chakra-ui/hooks"
import {Input} from "@chakra-ui/input"
import {Container, Heading, VStack} from "@chakra-ui/layout"
import React from "react"
import {CheckUserInfoContext, login} from "../utils/auth"
import {useMutation, useQueryClient} from "react-query"
import type {UserInfo} from "../utils/auth"
import {Alert, AlertIcon, AlertTitle, AlertDescription} from "@chakra-ui/alert"
import {InputGroup, InputRightElement} from "@chakra-ui/react"

type LoginDetails = {username: string, password: string}

export const Login = () => {
    const [username, setUsername] = React.useState("")
    const [password, setPassword] = React.useState("")
    const [showPassword, setShowPassword] = useBoolean(false)
    const queryClient = useQueryClient()
    const [, setCheckUserInfo] = React.useContext(CheckUserInfoContext)

    const mutation = useMutation<UserInfo, {message: string}, LoginDetails, unknown>(async ({username, password}) => {
        return await login(username, password)
    }, {
        onSuccess: user => {
            queryClient.setQueryData("currentUserInfo", user)
            setCheckUserInfo.on()
        }
    })

    return <Container maxW="container.sm">
        <VStack spacing={5} as="form" onSubmit={e => {
            e.preventDefault()
            mutation.mutate({username, password})
        }}>
            <Heading>Login</Heading>
            {mutation.isError && mutation.error && <Alert status="error">
                <AlertIcon />
                <AlertTitle mr={2}>Login Failed:</AlertTitle>
                <AlertDescription>{mutation.error.message}</AlertDescription>
            </Alert>}
            <Input placeholder="zID" type="text" value={username} onChange={(e) => setUsername(e.target.value)} disabled={mutation.isLoading} />
            <Input placeholder="Password" type={showPassword ? "text" : "password"} value={password} onChange={(e) => setPassword(e.target.value)} disabled={mutation.isLoading} />
            <Button type="submit" w="100%" isLoading={mutation.isLoading}>Login</Button>
        </VStack>
    </Container>
}