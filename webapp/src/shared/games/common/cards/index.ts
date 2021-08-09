export {default as CARD_BACK} from "./back.svg"

export type Suit = "H" | "D" | "C" | "S" | "u"
export type Rank = "A" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "10" | "J" | "Q" | "K" | "u"
export type CardName = `${Rank}${Suit}`

// Look, I don't like this either but it's the best I can be bothered to do.

import card10C from "./10C.svg"
import card10D from "./10D.svg"
import card10H from "./10H.svg"
import card10S from "./10S.svg"
import card10u from "./10u.svg"
import card2C from "./2C.svg"
import card2D from "./2D.svg"
import card2H from "./2H.svg"
import card2S from "./2S.svg"
import card2u from "./2u.svg"
import card3C from "./3C.svg"
import card3D from "./3D.svg"
import card3H from "./3H.svg"
import card3S from "./3S.svg"
import card3u from "./3u.svg"
import card4C from "./4C.svg"
import card4D from "./4D.svg"
import card4H from "./4H.svg"
import card4S from "./4S.svg"
import card4u from "./4u.svg"
import card5C from "./5C.svg"
import card5D from "./5D.svg"
import card5H from "./5H.svg"
import card5S from "./5S.svg"
import card5u from "./5u.svg"
import card6C from "./6C.svg"
import card6D from "./6D.svg"
import card6H from "./6H.svg"
import card6S from "./6S.svg"
import card6u from "./6u.svg"
import card7C from "./7C.svg"
import card7D from "./7D.svg"
import card7H from "./7H.svg"
import card7S from "./7S.svg"
import card7u from "./7u.svg"
import card8C from "./8C.svg"
import card8D from "./8D.svg"
import card8H from "./8H.svg"
import card8S from "./8S.svg"
import card8u from "./8u.svg"
import card9C from "./9C.svg"
import card9D from "./9D.svg"
import card9H from "./9H.svg"
import card9S from "./9S.svg"
import card9u from "./9u.svg"
import cardAC from "./AC.svg"
import cardAD from "./AD.svg"
import cardAH from "./AH.svg"
import cardAS from "./AS.svg"
import cardAu from "./Au.svg"
import cardJC from "./JC.svg"
import cardJD from "./JD.svg"
import cardJH from "./JH.svg"
import cardJS from "./JS.svg"
import cardJu from "./Ju.svg"
import cardKC from "./KC.svg"
import cardKD from "./KD.svg"
import cardKH from "./KH.svg"
import cardKS from "./KS.svg"
import cardKu from "./Ku.svg"
import cardQC from "./QC.svg"
import cardQD from "./QD.svg"
import cardQH from "./QH.svg"
import cardQS from "./QS.svg"
import cardQu from "./Qu.svg"
import carduC from "./uC.svg"
import carduD from "./uD.svg"
import carduH from "./uH.svg"
import carduS from "./uS.svg"
import carduu from "./uu.svg"

export const CARDS: Record<CardName, string> = {
    "10C": card10C,
    "10D": card10D,
    "10H": card10H,
    "10S": card10S,
    "10u": card10u,
    "2C": card2C,
    "2D": card2D,
    "2H": card2H,
    "2S": card2S,
    "2u": card2u,
    "3C": card3C,
    "3D": card3D,
    "3H": card3H,
    "3S": card3S,
    "3u": card3u,
    "4C": card4C,
    "4D": card4D,
    "4H": card4H,
    "4S": card4S,
    "4u": card4u,
    "5C": card5C,
    "5D": card5D,
    "5H": card5H,
    "5S": card5S,
    "5u": card5u,
    "6C": card6C,
    "6D": card6D,
    "6H": card6H,
    "6S": card6S,
    "6u": card6u,
    "7C": card7C,
    "7D": card7D,
    "7H": card7H,
    "7S": card7S,
    "7u": card7u,
    "8C": card8C,
    "8D": card8D,
    "8H": card8H,
    "8S": card8S,
    "8u": card8u,
    "9C": card9C,
    "9D": card9D,
    "9H": card9H,
    "9S": card9S,
    "9u": card9u,
    "AC": cardAC,
    "AD": cardAD,
    "AH": cardAH,
    "AS": cardAS,
    "Au": cardAu,
    "JC": cardJC,
    "JD": cardJD,
    "JH": cardJH,
    "JS": cardJS,
    "Ju": cardJu,
    "QC": cardQC,
    "QD": cardQD,
    "QH": cardQH,
    "QS": cardQS,
    "Qu": cardQu,
    "KC": cardKC,
    "KD": cardKD,
    "KH": cardKH,
    "KS": cardKS,
    "Ku": cardKu,
    "uC": carduC,
    "uD": carduD,
    "uH": carduH,
    "uS": carduS,
    "uu": carduu
}