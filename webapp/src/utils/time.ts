import format from "date-fns/format";
import formatDistance from "date-fns/formatDistance";

export const formatTimestamp = (timestamp: number) => ({
    relative: formatDistance(new Date(timestamp), new Date(), {addSuffix: true}),
    localised: format(new Date(timestamp), "Ppp")
})