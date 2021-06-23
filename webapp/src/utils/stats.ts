export const getOrdinalSuffix = (n: number) => {
    if (n >= 11 && n <= 13) {
        return "th"
    }

    const digit = n % 10

    if (digit === 1) {
        return "st"
    } else if (digit === 2) {
        return "nd"
    } else if (digit === 3) {
        return "rd"
    } else {
        return "th"
    }
}