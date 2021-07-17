import { Server, Origins } from "boardgame.io/server"
import { Round1Game } from "@shared/games/round-1/game";

const server = Server({
  games: [Round1Game],
  origins: [Origins.LOCALHOST],
});

server.run(8081);