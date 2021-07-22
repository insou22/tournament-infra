import {Container, VStack} from "@chakra-ui/react";
import React from "react";

export const VStackPageWrapper: React.FC<{spacing?: number}> = ({children, spacing}) => <Container maxW="container.lg">
    <VStack spacing={spacing || 4} alignItems="flex-start">
        {children}
    </VStack>
</Container>