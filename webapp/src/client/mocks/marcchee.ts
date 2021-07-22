import type {Binary, UserProfile} from "@client/api"

export const binaries: Binary[] = [
    {
        hash: "2678afd65ad",
        created_at: 1625273796054,
        compile_result: {status: "success", time_taken_ms: 96},
        stats_summary: {
            wins: 247,
            losses: 194,
            draws: 34,
            win_loss_ratio_percentage_change: 13.55,
            average_turn_run_time_ms: 623,
            average_turn_run_time_ms_percentage_change: -14.12
        }
    },
    {
        hash: "65d687f5fa",
        created_at: 1625273796054,
        compile_result: {status: "success", time_taken_ms: 96},
        stats_summary: {
            wins: 123,
            losses: 234,
            win_loss_ratio_percentage_change: 23.40,
            draws: 23,
            average_turn_run_time_ms: 957,
            average_turn_run_time_ms_percentage_change: 1
        }
    },
    {
        hash: "war3drwd4w",
        created_at: 1625273796054,
        compile_result: {status: "success", time_taken_ms: 96},
        stats_summary: {
            wins: 1234,
            losses: 243,
            win_loss_ratio_percentage_change: -10.40,
            draws: 2,
            average_turn_run_time_ms: 250,
            average_turn_run_time_ms_percentage_change: -14
        }
    }
]

export const userProfile: UserProfile = {
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
    current_binary: binaries[0]
}

