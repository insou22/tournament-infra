import axios from "axios"

export const api = axios.create({
    //@ts-expect-error Snowpack Env is provided by snowpack to access build-time environment variables.
    baseURL: __SNOWPACK_ENV__.SNOWPACK_API_URL,
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
    win_loss_ratio_percentage_change?: number
    average_turn_run_time_ms: number
    average_turn_run_time_ms_percentage_change?: number
}

export interface Tournament {
    name: string
}

export interface UserProfile {
    username: string
    display_name: string
    current_tournament_stats_summary: TournamentStats | null
    current_binary: Binary | null
}

export interface Binary {
    hash: string
    created_at: number
    compile_result: {
        status: "success"
        time_taken_ms: number
    } | {
        status: "failure"
        reason: "timed_out" | "error"
    } | null
    stats_summary: BinaryStats
}

export type Stream = "stdin" | "stdout" | "stderr"

export interface Turn {
    username: string,
    move: string,
    streams: Record<Stream, string>,
    run_time: number
}

export type PlayerResult = "won" | "lost" | "drew"

export interface Player {
    binary_hash: string
    username: string,
    display_name: string
    elo_before_game: number,
    elo_change: number
    result: PlayerResult
}

export interface Game {
    id: number
    created_at: number
    completed_at: number | null
    players: Player[]
}

export interface Ranking {
    username: string
    display_name: string
    rating: number
    win_loss: number
}