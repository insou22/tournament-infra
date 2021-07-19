import type {Game} from "boardgame.io"
import {Round1Board, Round1Game} from "./round-1";
import type {BoardProps} from "boardgame.io/dist/types/packages/react";
import type React from "react";

export const GAMES: Record<string, {board: React.FC<BoardProps>, game: Game}> = {
    "round-1": {
        board: Round1Board,
        game: Round1Game
    }
}