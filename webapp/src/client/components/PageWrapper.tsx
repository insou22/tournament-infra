import {Box, BoxProps, Container, VStack} from "@chakra-ui/react"
import {motion} from "framer-motion"
import React from "react"

const MotionBox = motion<BoxProps>(Box)

const pageTransitionVariants = {
    hidden: {
        opacity: 0
    },
    visible: {
        opacity: 1
    }
}

const PageTransitionWrapper: React.FC = (props) => <MotionBox initial="hidden" animate="visible" exit="hidden" variants={pageTransitionVariants}>
    {props.children}
</MotionBox>

const VStackPageWrapper: React.FC<{spacing?: number}> = ({children, spacing}) => <Container maxW="container.lg">
    <VStack spacing={spacing || 4} alignItems="flex-start">
        {children}
    </VStack>
</Container>

export const PageWrapper: React.FC<{spacing?: number}> = ({children, ...props}) => <PageTransitionWrapper>
    <VStackPageWrapper {...props}>
        {children}
    </VStackPageWrapper>
</PageTransitionWrapper>