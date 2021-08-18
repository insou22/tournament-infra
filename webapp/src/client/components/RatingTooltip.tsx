import {Tooltip} from "@chakra-ui/react"
import React from "react"

export const RatingTooltip: React.FC = ({children}) => <Tooltip hasArrow label="A rating is made up of μ and σ values. μ represents the current estimate of a player's true rating, and σ is a measure of confidence in that rating. Ratings are shown as μ±3σ">
    {children}
</Tooltip>