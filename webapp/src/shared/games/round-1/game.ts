import type {Suit} from "@shared/common/cards";
import type {Ctx, Game} from "boardgame.io"

type Card = {suit: Suit, rank: number}
type PlayerState = {score: number, hand: Card[], played: Card | null}
type Player = "0" | "1"
type State = {trickStarter: Player, players: Record<Player, PlayerState>};
type SetupData = "";

const SUITS: Suit[] = ["H", "D", "C", "S"]
const CARD_NUMBERS = [1, 2, 3, 4, 5]
const STARTING_DECK: Card[] = SUITS.flatMap(suit => CARD_NUMBERS.map(rank => ({suit, rank})))

export const Round1Game: Game<State, Ctx, SetupData> = {
    setup: (ctx): State => {
        const deck = ctx.random!.Shuffle(STARTING_DECK);

        return {
            trickStarter: "0",
            players: {
                "0": {
                    score: 0,
                    hand: deck.slice(0, 10),
                    played: null
                },
                "1": {
                    score: 0,
                    hand: deck.slice(0, 10),
                    played: null
                }
            }
        }
    },

    endIf: G => G.players["0"].score + G.players["1"].score === 10,

    phases: {
        trick: {
            start: true,
            next: "trick",
            turn: {
                moveLimit: 1,
                order: {
                    first: G => Number(G.trickStarter),
                    next: (_, ctx) => ctx.currentPlayer === "0" ? 1 : 0
                }
            },
            moves: {
                playCard: (G, ctx, handIndex: number) => {
                    const [card] = G.players[ctx.currentPlayer as Player].hand.splice(handIndex)
                    G.players[ctx.currentPlayer as Player].played = card
                },
            },
            endIf: G => !!(G.players["0"].played && G.players["1"].played),
            onEnd: (G, ctx) => {
                if (G.players["0"].played!.suit === G.players["1"].played!.suit) {
                    if (G.players["0"].played!.rank > G.players["1"].played!.rank) {
                        G.trickStarter = "0"
                    } else {
                        G.trickStarter = "1"
                    }
                }

                G.players[G.trickStarter].score++
            }
        }
    }
};