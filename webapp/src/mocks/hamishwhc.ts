import type {Binary, UserProfile} from "src/api"

export const binaries: Binary[] = [
    {
        hash: "3132as2323d",
        created_at: 1625273796054,
        compile_result: {status: "success", time_taken_ms: 96},
        stats_summary: {
            wins: 234,
            losses: 543,
            draws: 234,
            win_loss_ratio_percentage_change: -13.56,
            average_turn_run_time_ms: 2356,
            average_turn_run_time_ms_percentage_change: -34.21
        }
    },
    {
        hash: "67s5f678sdg67",
        created_at: 1625273796054,
        compile_result: {status: "success", time_taken_ms: 96},
        stats_summary: {
            wins: 433,
            losses: 563,
            win_loss_ratio_percentage_change: 12.85,
            draws: 23,
            average_turn_run_time_ms: 787,
            average_turn_run_time_ms_percentage_change: 1.93
        }
    },
    {
        hash: "fse568sef",
        created_at: 1625273796054,
        compile_result: {status: "success", time_taken_ms: 96},
        stats_summary: {
            wins: 345,
            losses: 312,
            win_loss_ratio_percentage_change: -12.34,
            draws: 2,
            average_turn_run_time_ms: 261,
            average_turn_run_time_ms_percentage_change: -14.28
        }
    }
]

export const userProfile: UserProfile = {
    username: "HamishWHC",
    display_name: "Hamish Cox",
    current_tournament_stats_summary: {
        ranking: 5,
        wins: 1134,
        losses: 578,
        draws: 97,
        elo: 1123,
        average_turn_run_time_ms: 345
    },
    current_binary: binaries[0]
}

