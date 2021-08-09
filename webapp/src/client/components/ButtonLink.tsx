import {Button, ButtonProps} from "@chakra-ui/react"
import React from "react"
import {useHistory} from "react-router-dom"

export const ButtonLink: React.FC<{href: string, size?: string} & ButtonProps> = ({href, children, size, ...props}) => {
    const history = useHistory()
    return <Button variant="link" size={size} onClick={() => history.push(href)} w="min-content" {...props}>
        {children}
    </Button>
}