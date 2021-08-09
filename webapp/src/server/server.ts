import {game as round1Game} from "@shared/games/round-1/game"
import {Origins, Server} from "boardgame.io/server"

const server = Server({
    games: [round1Game],
    origins: [process.env.ORIGIN || Origins.LOCALHOST_IN_DEVELOPMENT],
})

server.run(8081)