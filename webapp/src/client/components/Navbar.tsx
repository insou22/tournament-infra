import {Button, IconButton} from "@chakra-ui/button"
import {PlusSquareIcon, SettingsIcon} from "@chakra-ui/icons"
import {Box, HStack, Text, VStack} from "@chakra-ui/layout"
import {Modal, ModalBody, ModalCloseButton, ModalContent, ModalFooter, ModalHeader, ModalOverlay, Spinner, useDisclosure} from "@chakra-ui/react"
import React from "react"
import {useHistory, useRouteMatch} from "react-router-dom"
import {useUserInfo} from "@client/hooks/useUserInfo"
import {ButtonLink} from "./ButtonLink"
import {FileUpload} from "./FileUpload"

export const Navbar = () => {
    const {user, isLoading} = useUserInfo()
    const disclosure = useDisclosure()
    const history = useHistory();

    let userControls;
    if (isLoading) {
        userControls = <Spinner size="sm" />
    } else if (user) {
        userControls = <HStack spacing={2}>
            <VStack alignItems="flex-end" spacing={1}>
                <ButtonLink size="md" href="/profile">{user.display_name}</ButtonLink>
                <Text fontSize="xs">{user.current_elo ? `${user.current_elo}` : "Unrated"}</Text>
            </VStack>
            <NavbarIconLink aria-label="settings" path="/settings" icon={<SettingsIcon />} exact={false} />
        </HStack>
    } else {
        userControls = <NavbarLink text="Login" path="/login" exact={true} />
    }

    return <>
        <HStack justifyContent="space-between" alignItems="center">
            <Box display="flex" alignItems="center" onClick={() => history.push("/")} cursor="pointer">
                <img src="https://i.ytimg.com/vi/bDByGe7FgEQ/hqdefault.jpg" height="50" width="50" />
                <Text fontSize="lg" ml={2} fontWeight="bold">MarcComp</Text>
            </Box>
            <HStack>
                <NavbarLink text="About" path="/about" exact={false} />
                <NavbarLink text="FAQs" path="/faqs" exact={false} />
                <NavbarLink text="Spec" path="/spec" exact={false} />
                <NavbarLink text="Rankings" path="/rankings" exact={false} />
                <NavbarLink text="Games" path="/games" exact={false} />
                <NavbarLink text="Play" path="/play" exact={false} />
                <Button leftIcon={<PlusSquareIcon />} variant="ghost" onClick={disclosure.onOpen}>Upload Code</Button>
            </HStack>
            {userControls}
        </HStack>
        <CodeUploadModal {...disclosure} />
    </>
}

const NavbarLink = ({text, path, exact, ...rest}: {text: string, path: string, exact: boolean}) => {
    const history = useHistory()
    const match = useRouteMatch({path, exact})

    return <Button onClick={() => history.push(path)} variant={match ? "solid" : "ghost"} {...rest}>{text}</Button>
}

const NavbarIconLink = ({path, exact, icon, "aria-label": label, ...rest}: {"aria-label": string, path: string, exact: boolean, icon: React.ReactElement<any, string | React.JSXElementConstructor<any>>}) => {
    const history = useHistory()
    const match = useRouteMatch({path, exact})

    return <IconButton rounded="md" aria-label={label} onClick={() => history.push(path)} variant={match ? "solid" : "ghost"} icon={icon} {...rest} />
}

const CodeUploadModal = ({isOpen, onClose}: {isOpen: boolean, onClose: () => void}) => {
    const [file, setFile] = React.useState<File | null>(null)

    return <Modal isOpen={isOpen} onClose={onClose}>
        <ModalOverlay />
        <ModalContent>
            <ModalHeader>Code Upload</ModalHeader>
            <ModalCloseButton />
            <ModalBody>
                <HStack spacing={4}>
                    <FileUpload acceptedFileTypes=".c" onChange={setFile} />
                    <Text>{file ? file.name : "No File Selected"}</Text>
                </HStack>
            </ModalBody>

            <ModalFooter>
                <Button colorScheme="green" mr={3} onClick={onClose} disabled={!file}>
                    Upload
                </Button>
            </ModalFooter>
        </ModalContent>
    </Modal>
}