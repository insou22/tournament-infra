import {Heading, Text} from "@chakra-ui/layout"
import {ListItem, UnorderedList} from "@chakra-ui/react"
import {PageWrapper} from "@client/components/PageWrapper"
import React from "react"
import ReactMarkdown from 'react-markdown'
import type {NormalComponents, SpecialComponents} from "react-markdown/src/ast-to-react"
import remarkGfm from "remark-gfm"
import spec from "./spec-markdown.md"

const components: Partial<NormalComponents & SpecialComponents> = {
    h1: props => <Heading size="xl">{props.children}</Heading>,
    h2: props => <Heading size="lg" pt={3}>{props.children}</Heading>,
    h3: props => <Heading size="md" pt={2}>{props.children}</Heading>,
    h4: props => <Heading size="sm" pt={1}>{props.children}</Heading>,
    h5: props => <Heading size="xs">{props.children}</Heading>,
    h6: props => <Text fontWeight="bold">{props.children}</Text>,
    p: props => <Text>{props.children}</Text>,
    ul: props => <UnorderedList stylePosition="inside" pl={4}>{props.children}</UnorderedList>,
    li: props => <ListItem>{props.children}</ListItem>
}

export const Spec = () => {
    return <PageWrapper spacing={2}>
        <ReactMarkdown plugins={[remarkGfm]} components={components} children={spec.markdown} />
    </PageWrapper>
}