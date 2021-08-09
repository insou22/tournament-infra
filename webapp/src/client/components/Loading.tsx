import {Text, VStack} from "@chakra-ui/react"
import {Spinner} from "@chakra-ui/spinner"
import React from "react"

export const Loading = ({centered, text}: {centered?: boolean, text?: string}) => <VStack w={centered ? "100%" : undefined} justifyContent="center" spacing={4}>
    {text && <Text>{text}</Text>}
    <Spinner size="xl" />
</VStack>