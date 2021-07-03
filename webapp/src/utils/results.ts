import type {PlayerResult} from "src/api";

export const resultProps: Record<PlayerResult, {label: string, color: string}> = {
    won: {
        label: "Won",
        color: "green"
    },
    lost: {
        label: "Lost",
        color: "red"
    },
    drew: {
        label: "Drew",
        color: "purple"
    }
}