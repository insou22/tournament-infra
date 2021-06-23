import axios from "axios"

export const api = axios.create({
    baseURL: "http://127.0.0.1:8000/",
    withCredentials: true
})

export interface TournamentStats {
    ranking: number
    wins: number
    losses: number
    draws: number
    elo: number
    average_turn_run_time_ms: number
}

export interface BinaryStats {
    wins: number
    losses: number
    draws: number
    win_loss_ratio_percentage_change: number
    average_turn_run_time_ms: number
    average_turn_run_time_ms_percentage_change: number
}

export interface UserProfile {
    username: string
    display_name: string
    current_tournament_stats_summary: TournamentStats
    current_binary_stats_summary: BinaryStats
}