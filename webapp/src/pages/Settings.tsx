import {Heading} from "@chakra-ui/layout"
import {Button} from "@chakra-ui/react"
import React from "react"
import {useMutation} from "react-query"
import {VStackPageWrapper} from "src/components/VStackPageWrapper"
import {CheckUserInfoContext, logout} from "src/utils/auth"

export const Settings = () => {
    const [, setCheckUserInfo] = React.useContext(CheckUserInfoContext)

    const logoutMutation = useMutation(async () => {
        return await logout()
    }, {
        onSuccess: () => setCheckUserInfo.off()
    })

    return <VStackPageWrapper>
        <Heading>Settings</Heading>
        <Button variant="solid" colorScheme="red" onClick={() => logoutMutation.mutate()}>Logout</Button>
    </VStackPageWrapper>
}