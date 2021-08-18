export const displayRating = (mu: number, sigma: number) => {
    return `${mu.toFixed(2)} ± ${(3*sigma).toFixed(2)}`
}