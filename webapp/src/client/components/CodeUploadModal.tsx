import {HStack} from "@chakra-ui/layout"
import {Button, Modal, ModalBody, ModalCloseButton, ModalContent, ModalFooter, ModalHeader, ModalOverlay, Text} from "@chakra-ui/react"
import {api, Binary} from "@client/api"
import {useUserInfo} from "@client/hooks/useUserInfo"
import React from "react"
import {useMutation} from "react-query"
import {useHistory} from "react-router-dom"
import {FileUpload} from "./FileUpload"

export const CodeUploadModal = ({isOpen, onClose}: {isOpen: boolean, onClose: () => void}) => {
    const [file, setFile] = React.useState<File | null>(null)
    const {user} = useUserInfo()
    const history = useHistory()

    const uploadMutation = useMutation<Binary, unknown, File, unknown>(async (file: File) => {
        const formData = new FormData()
        formData.append("file", file)
        return (await api.put("/binaries", formData)).data
    }, {
        onSuccess: binary => {
            history.push(`/user/${user!.username}/binary/${binary.hash}`)
            onClose()
        }
    })

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
                <Button colorScheme="green" mr={3} onClick={() => uploadMutation.mutate(file!)} disabled={!file} isLoading={uploadMutation.isLoading}>
                    Upload
                </Button>
            </ModalFooter>
        </ModalContent>
    </Modal>
}