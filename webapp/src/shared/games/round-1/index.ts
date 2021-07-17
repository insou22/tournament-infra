import {Client} from 'boardgame.io/react';
import {SocketIO} from 'boardgame.io/multiplayer';
import {Round1Game} from './game';
import {Round1Board} from './Board';

export const Round1Client = Client({
    game: Round1Game,
    board: Round1Board,
    multiplayer: SocketIO({server: "localhost:8081"}),
});