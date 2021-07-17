import type {Game} from "@client/api";
import * as marcchee from "./marcchee"
import * as hamishwhc from "./hamishwhc"

export const marcVsHamish: Game = {
    id: 1,
    created_at: 1625273796054,
    completed_at: 1625273796054,
    players: [
        {
            ...marcchee.userProfile,
            binary_hash: marcchee.binaries[2].hash,
            elo_before_game: 2376,
            elo_change: 23,
            result: "won"
        },
        {
            ...hamishwhc.userProfile,
            binary_hash: hamishwhc.binaries[2].hash,
            elo_before_game: 1168,
            elo_change: -13,
            result: "lost"
        }
    ]
}

export const marcVsHamish2: Game = {
    id: 2,
    created_at: 1625273796054,
    completed_at: 1625273796054,
    players: [
        {
            ...hamishwhc.userProfile,
            binary_hash: hamishwhc.binaries[1].hash,
            elo_before_game: 2324,
            elo_change: 0,
            result: "drew"
        },
        {
            ...marcchee.userProfile,
            binary_hash: marcchee.binaries[0].hash,
            elo_before_game: 1237,
            elo_change: 0,
            result: "drew"
        }
    ]
}

export const allGames: Game[] = [
    marcVsHamish,
    marcVsHamish2
]