import {Container, VStack} from "@chakra-ui/react";
import React from "react";

export const VStackPageWrapper: React.FC = ({children}) => <Container maxW="container.lg">
    <VStack spacing={4} alignItems="flex-start">
        {children}
    </VStack>
</Container>