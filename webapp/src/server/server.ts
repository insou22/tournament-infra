import { Server, Origins } from "boardgame.io/server"
import {game as round1Game} from "@shared/games/round-1/game";

const server = Server({
  games: [round1Game],
  origins: [process.env.ORIGIN || Origins.LOCALHOST_IN_DEVELOPMENT],
});

server.run(8081);