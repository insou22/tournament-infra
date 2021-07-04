import {Heading, Text} from "@chakra-ui/layout"
import {ListItem, UnorderedList} from "@chakra-ui/react"
import React from "react"
import ReactMarkdown from 'react-markdown'
import type {NormalComponents, SpecialComponents} from "react-markdown/src/ast-to-react"
import remarkGfm from "remark-gfm"
import {VStackPageWrapper} from "src/components/VStackPageWrapper"
//@ts-expect-error TS doesn't know we have markdown import capabilities.
import spec from "./spec.md"

const components: Partial<NormalComponents & SpecialComponents> = {
    h1: props => <Heading size="xl">{props.children}</Heading>,
    h2: props => <Heading size="lg">{props.children}</Heading>,
    h3: props => <Heading size="md">{props.children}</Heading>,
    h4: props => <Heading size="sm">{props.children}</Heading>,
    h5: props => <Heading size="xs">{props.children}</Heading>,
    h6: props => <Text fontWeight="bold">{props.children}</Text>,
    p: props => <Text>{props.children}</Text>,
    ul: props => <UnorderedList stylePosition="inside">{props.children}</UnorderedList>,
    li: props => <ListItem>{props.children}</ListItem>
}

export const Spec = () => {
    return <VStackPageWrapper spacing={2}>
        <ReactMarkdown plugins={[remarkGfm]} components={components} children={spec.markdown} />
    </VStackPageWrapper>
}