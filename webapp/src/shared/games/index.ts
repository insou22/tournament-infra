import type {Game} from "boardgame.io"
import Round1 from "./round-1";
import type {BoardProps} from "boardgame.io/dist/types/packages/react";
import type React from "react";

export const GAMES: Record<string, {board: React.FC<BoardProps>, game: Game}> = {
    [Round1.game.name!]: Round1
}