import {useBoolean} from "@chakra-ui/hooks"
import React from "react"
import {useHistory} from "react-router"
import {UserContext} from "src/UserContext"

export const LoginPage = () => {
    const history = useHistory()
    const {user, setUser} = React.useContext(UserContext)
    const [loading, setLoading] = useBoolean(false)

    const loginOnClick = React.useCallback(async () => {
        setLoading(true)
        await login()
        setLoading(false)
    }, [history])

    return <>
        Test Code Hi
    </>
}