import {Heading} from "@chakra-ui/layout"
import React from "react"

export const Profile = ({username}: {username: string}) => {
    return <Heading>{username}</Heading>
}