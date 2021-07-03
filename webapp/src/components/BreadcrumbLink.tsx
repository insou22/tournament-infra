import React from "react"
import {BreadcrumbLink as ChakraBreadcrumbLink, Button} from "@chakra-ui/react"
import {useHistory} from "react-router-dom"

const BreadcrumbLinkButton: React.FC<{href: string}> = ({href, children}) => {
    const history = useHistory()
    return <Button variant="link" onClick={() => history.push(href)}>
        {children}
    </Button>
}

export const BreadcrumbLink: React.FC<{href: string}> = ({children, href}) => <ChakraBreadcrumbLink as={BreadcrumbLinkButton} href={href}>
    {children}
</ChakraBreadcrumbLink>