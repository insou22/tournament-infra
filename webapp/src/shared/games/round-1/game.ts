import type {Suit} from "@shared/games/common/cards";
import type {Ctx, Game, PlayerID} from "boardgame.io"
import type {BaseSetupData} from "../types";

type Card = {suit: Suit, rank: number}
type PlayerState = {score: number, hand: Card[], played: Card | null}
type State = {trickStarter: PlayerID, players: Record<PlayerID, PlayerState>};

const SUITS: Suit[] = ["H", "D", "C", "S"]
const CARD_NUMBERS = [1, 2, 3, 4, 5]
const STARTING_DECK: Card[] = SUITS.flatMap(suit => CARD_NUMBERS.map(rank => ({suit, rank})))

interface SetupData extends BaseSetupData {
    startingPlayer?: PlayerID
}

export const Round1Game: Game<State, Ctx, SetupData> = {
    name: "round-1",
    maxPlayers: 2,
    minPlayers: 2,
    disableUndo: true,
    
    setup: (ctx, setupData): State => {
        const deck = ctx.random!.Shuffle(STARTING_DECK);

        return {
            trickStarter: setupData?.startingPlayer || ctx.currentPlayer,
            players: {
                "0": {
                    score: 0,
                    hand: deck.slice(0, 10),
                    played: null
                },
                "1": {
                    score: 0,
                    hand: deck.slice(10, 20),
                    played: null
                }
            }
        }
    },

    moves: {
        playCard: (G, ctx, handIndex: number) => {
            const [card] = G.players[ctx.currentPlayer].hand.splice(handIndex, 1)
            G.players[ctx.currentPlayer].played = card
        },
    },

    turn: {
        order: {
            first: G => Number(G.trickStarter),
            next: G => {
                if (G.players["0"].played || G.players["1"].played) {
                    return Number(!Number(G.trickStarter))
                } else {
                    return Number(G.trickStarter)
                }
            }
        },
        moveLimit: 1,
        onEnd: G => {
            if (G.players["0"].played && G.players["1"].played) {
                if (G.players["0"].played!.suit === G.players["1"].played!.suit) {
                    if (G.players["0"].played!.rank > G.players["1"].played!.rank) {
                        G.trickStarter = "0"
                    } else {
                        G.trickStarter = "1"
                    }
                }

                G.players[G.trickStarter].score++

                G.players["0"].played = null
                G.players["1"].played = null
            }
        }
    },

    endIf: G => G.players["0"].score + G.players["1"].score === 10
};