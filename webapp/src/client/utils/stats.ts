const defaultRankingStyles = {suffix: "th", color: "white", decoration: null}

export const getRankingStyles = (n: number) => {
    if (n == 1) {
        return {
            suffix: "st",
            color: "gold",
            decoration: "🥇"
        }
    } else if (n == 2) {
        return {
            suffix: "nd",
            color: "silver",
            decoration: "🥈"
        }
    } else if (n == 3) {
        return {
            suffix: "rd",
            color: "cd7f32",
            decoration: "🥉"
        }
    }

    if (n >= 11 && n <= 13) {
        return defaultRankingStyles
    }

    const digit = n % 10

    if (digit === 1) {
        return {...defaultRankingStyles, suffix: "st"}
    } else if (digit === 2) {
        return {...defaultRankingStyles, suffix: "nd"}
    } else if (digit === 3) {
        return {...defaultRankingStyles, suffix: "rd"}
    } 

    return defaultRankingStyles
}