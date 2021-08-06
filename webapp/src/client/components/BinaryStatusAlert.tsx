import {Alert, AlertIcon, AlertTitle, AlertDescription} from "@chakra-ui/react"
import type {CompileResult} from "@client/api"
import React from "react"

export const BinaryStatusAlert = ({result}: {result: Exclude<CompileResult, "success">}) => {
    if (result === "not_compiled") {
        return <Alert status="info">
            <AlertIcon />
            <AlertTitle>Queued for compilation.</AlertTitle>
            <AlertDescription>
                This binary has not been compiled yet. Check back in a few minutes.
            </AlertDescription>
        </Alert>
    } else {
        const title = result === "failed" ? "Compilation failed." : "Compilation timed out."
        const description = result === "failed" ? "" : ""
        return <Alert status="error">
            <AlertIcon />
            <AlertTitle>{title}</AlertTitle>
            <AlertDescription>{description}</AlertDescription>
        </Alert>
    }
}