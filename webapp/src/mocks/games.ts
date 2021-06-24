import type {Game} from "src/api";
import * as marcchee from "./marcchee"
import * as hamishwhc from "./hamishwhc"

export const marcVsHamish: Game = {
    players: [
        {
            user_profile: marcchee.userProfile,
            binary: marcchee.binaries[2],
            elo_before_game: 2376,
            elo_change: 23,
            result: "won"
        },
        {
            user_profile: hamishwhc.userProfile,
            binary: hamishwhc.binaries[2],
            elo_before_game: 1168,
            elo_change: -13,
            result: "lost"
        }
    ]
}