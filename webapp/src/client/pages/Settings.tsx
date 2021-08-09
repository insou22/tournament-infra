import {Heading} from "@chakra-ui/layout"
import {Button, FormControl, FormLabel, HStack, Input} from "@chakra-ui/react"
import {api, UserProfile} from "@client/api"
import {VStackPageWrapper} from "@client/components/VStackPageWrapper"
import {useUserInfo} from "@client/hooks/useUserInfo"
import {CheckUserInfoContext, logout} from "@client/utils/auth"
import type {AxiosError} from "axios"
import React from "react"
import {useMutation, useQueryClient} from "react-query"

export type UserProfilePatch = Pick<UserProfile, "display_name">

export const Settings = () => {
    const queryClient = useQueryClient()
    const [, setCheckUserInfo] = React.useContext(CheckUserInfoContext)
    const userInfo = useUserInfo()
    const [formData, setFormData] = React.useState<UserProfilePatch>({display_name: ""})
    // const {colorMode, setColorMode} = useColorMode()

    React.useEffect(() => {
        setFormData(prev => ({
            ...prev,
            display_name: userInfo.user!.display_name
        }))
    }, [userInfo.user])

    const logoutMutation = useMutation(async () => {
        return await logout()
    }, {
        onSuccess: () => setCheckUserInfo.off()
    })

    const userProfileMutation = useMutation<UserProfile, AxiosError, {profilePatch: Partial<UserProfilePatch>, username: string}>(async ({profilePatch}) => {
        const res = await api.patch(`/user`, profilePatch)
        return res.data
    }, {
        onSuccess: (_data, vars) => {
            queryClient.invalidateQueries(["userProfile", vars.username])
            queryClient.invalidateQueries("currentUserInfo")
        }
    })

    return <VStackPageWrapper>
        <HStack w="100%" justifyContent="space-between">
            <Heading>Settings</Heading>
            <Button variant="solid" colorScheme="red" onClick={() => logoutMutation.mutate()} isLoading={logoutMutation.isLoading}>Logout</Button>
        </HStack>
        <HStack alignItems="flex-end">
            <FormControl>
                <FormLabel>Display Name</FormLabel>
                <Input placeholder={userInfo.user!.username} w="xs" value={formData.display_name} onChange={(e) => setFormData(p => ({...p, display_name: e.target.value}))} disabled={userProfileMutation.isLoading} />
            </FormControl>
            <Button variant="solid" colorScheme="green" onClick={() => userProfileMutation.mutate({profilePatch: formData, username: userInfo.user!.username})} isLoading={userProfileMutation.isLoading}>Save</Button>
        </HStack>
        {/* <FormControl>
            <FormLabel>Appearance</FormLabel>
            <Select w="xs" value={colorMode} onChange={(e) => setColorMode(e.target.value)}>
                <option value="system">System</option>
                <option value="light">Light</option>
                <option value="dark">Dark</option>
            </Select>
        </FormControl> */}
    </VStackPageWrapper>
}