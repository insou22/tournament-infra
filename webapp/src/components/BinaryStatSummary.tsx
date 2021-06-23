import React from "react"
import type {BinaryStats} from "src/api"
import {StatsSummary} from "./StatSummary"

export const BinaryStatsSummary = ({stats}: {stats: BinaryStats}) => {
    return <StatsSummary stats={[
        {
            label: "Wins",
            value: stats.wins
        },
        {
            label: "Losses",
            value: stats.losses
        },
        {
            label: "W/L",
            value: (stats.wins / stats.losses).toFixed(2),
            change: stats.win_loss_ratio_percentage_change
        },
        {
            label: "Draws",
            value: stats.draws
        },
        {
            label: "Average Turn Run Time",
            value: `${stats.average_turn_run_time_ms}ms`,
            change: stats.average_turn_run_time_ms_percentage_change
        }
    ]} />
}