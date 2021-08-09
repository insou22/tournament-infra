import {LinkIcon} from "@chakra-ui/icons"
import {IconButton} from "@chakra-ui/react"
import React from "react"

export const FileUpload = ({acceptedFileTypes, onChange}: {acceptedFileTypes: string, onChange: (filename: File | null) => void}) => {
    const inputRef = React.useRef<HTMLInputElement>(null)
    return <>
        <input type='file' accept={acceptedFileTypes} ref={inputRef} style={{display: 'none'}} onChange={e => onChange(e.target.files ? e.target.files[0] : null)}></input>
        <IconButton
            rounded="md"
            aria-label="Upload File"
            icon={<LinkIcon />}
            onClick={() => inputRef.current?.click()}
            style={{margin: 0}}
        />
    </>
}