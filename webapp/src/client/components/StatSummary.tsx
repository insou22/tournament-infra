import {Box} from "@chakra-ui/layout"
import {Stat, StatArrow, StatGroup, StatHelpText, StatLabel, StatNumber} from "@chakra-ui/stat"
import React from "react"

export const StatsSummary = ({stats}: {
    stats: {
        label: string,
        value: number | string,
        subvalue?: number | string,
        change?: number,
        invert?: boolean
    }[]
}) => {
    return <Box rounded="xl" w="100%" bg="whiteAlpha.100">
        <StatGroup p={2} gridColumnGap={4}>
            {stats.map((stat, i) => <Stat key={i}>
                <StatLabel>{stat.label}</StatLabel>
                <StatNumber>{stat.value}</StatNumber>
                {stat.subvalue !== undefined && stat.subvalue !== null && <StatHelpText>{stat.subvalue}</StatHelpText>}
                {stat.change !== undefined && stat.change !== null && <StatHelpText>
                    <StatArrow type={stat.change < 0 !== stat.invert ? "decrease" : "increase"} transform={stat.invert ? "rotate(180deg)" : ""}/>
                    {stat.change.toFixed(2)}%
                </StatHelpText>}
            </Stat>)}
        </StatGroup>
    </Box>
}