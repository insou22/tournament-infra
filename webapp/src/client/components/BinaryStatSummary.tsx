import type {BinaryStats} from "@client/api"
import React from "react"
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
            value: (stats.win_loss ?? Infinity).toFixed(2),
            change: stats.win_loss_ratio_percentage_change
        },
        {
            label: "Draws",
            value: stats.draws
        },
        {
            label: "Average Turn Run Time",
            value: `${stats.average_turn_run_time_ms.toFixed(2)}ms`,
            change: stats.average_turn_run_time_ms_percentage_change,
            invert: true
        }
    ]} />
}