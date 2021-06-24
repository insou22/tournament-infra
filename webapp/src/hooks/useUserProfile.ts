import type {AxiosError} from "axios";
import {QueryFunction, useQuery} from "react-query";
import {api, UserProfile} from "src/api";
import {dontRetryOn404} from "src/utils/api";

const getUserProfile: QueryFunction<UserProfile, ["userProfile", string]> = async ({queryKey: [, username]}) => {
    return {
        username: "marcchee",
        display_name: "Marc Chee",
        current_tournament_stats_summary: {
            ranking: 4,
            wins: 247,
            losses: 194,
            draws: 34,
            elo: 1534,
            average_turn_run_time_ms: 623
        },
        current_binary: {
            hash: "2678afd65ad",
            created_at: "2021-06-23T23:12:45Z",
            stats_summary: {
                wins: 247,
                losses: 194,
                draws: 34,
                win_loss_ratio_percentage_change: 13.55,
                average_turn_run_time_ms: 623,
                average_turn_run_time_ms_percentage_change: -14.12
            }
        }
    }

    const response = await api.get<UserProfile>(`/user/${username}`)
    return response.data
}

export const useUserProfile = (username: string) => useQuery<unknown, AxiosError, UserProfile, ["userProfile", string]>(["userProfile", username], getUserProfile, {
    retry: dontRetryOn404,
})