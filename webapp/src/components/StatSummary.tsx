import {Box} from "@chakra-ui/layout"
import {StatGroup, Stat, StatLabel, StatNumber, StatHelpText, StatArrow} from "@chakra-ui/stat"
import React from "react"

export const StatsSummary = ({stats}: {
    stats: {
        label: string,
        value: number | string,
        change?: number
    }[]
}) => {
    return <Box borderColor="whiteAlpha.300" borderStyle="solid" borderWidth="1px" borderRadius={12} w="100%">
        <StatGroup p={2}>
            {stats.map((stat, i) => <Stat key={i}>
                <StatLabel>{stat.label}</StatLabel>
                <StatNumber>{stat.value}</StatNumber>
                {stat.change !== undefined && <StatHelpText>
                    <StatArrow type={stat.change < 0 ? "decrease" : "increase"} />
                    {stat.change}%
                </StatHelpText>}
            </Stat>)}
        </StatGroup>
    </Box>
}