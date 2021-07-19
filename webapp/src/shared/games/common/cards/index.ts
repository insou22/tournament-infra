export {default as CARD_BACK} from "./back.svg"

export type Suit = "H" | "D" | "C" | "S" | "u"
export type Rank = "A" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "10" | "J" | "Q" | "K" | "u"
export type CardName = `${Rank}${Suit}`

// Look, I don't like this either but it's the best I can be bothered to do.

import cardAC from "./AC.svg"
import card2C from "./2C.svg"
import card3C from "./3C.svg"
import card4C from "./4C.svg"
import card5C from "./5C.svg"
import card6C from "./6C.svg"
import card7C from "./7C.svg"
import card8C from "./8C.svg"
import card9C from "./9C.svg"
import card10C from "./10C.svg"
import cardJC from "./JC.svg"
import cardQC from "./QC.svg"
import cardKC from "./KC.svg"

import cardAH from "./AH.svg"
import card2H from "./2H.svg"
import card3H from "./3H.svg"
import card4H from "./4H.svg"
import card5H from "./5H.svg"
import card6H from "./6H.svg"
import card7H from "./7H.svg"
import card8H from "./8H.svg"
import card9H from "./9H.svg"
import card10H from "./10H.svg"
import cardJH from "./JH.svg"
import cardQH from "./QH.svg"
import cardKH from "./KH.svg"

import cardAS from "./AS.svg"
import card2S from "./2S.svg"
import card3S from "./3S.svg"
import card4S from "./4S.svg"
import card5S from "./5S.svg"
import card6S from "./6S.svg"
import card7S from "./7S.svg"
import card8S from "./8S.svg"
import card9S from "./9S.svg"
import card10S from "./10S.svg"
import cardJS from "./JS.svg"
import cardQS from "./QS.svg"
import cardKS from "./KS.svg"

import cardAD from "./AD.svg"
import card2D from "./2D.svg"
import card3D from "./3D.svg"
import card4D from "./4D.svg"
import card5D from "./5D.svg"
import card6D from "./6D.svg"
import card7D from "./7D.svg"
import card8D from "./8D.svg"
import card9D from "./9D.svg"
import card10D from "./10D.svg"
import cardJD from "./JD.svg"
import cardQD from "./QD.svg"
import cardKD from "./KD.svg"

import cardAu from "./Au.svg"
import card2u from "./2u.svg"
import card3u from "./3u.svg"
import card4u from "./4u.svg"
import card5u from "./5u.svg"
import card6u from "./6u.svg"
import card7u from "./7u.svg"
import card8u from "./8u.svg"
import card9u from "./9u.svg"
import card10u from "./10u.svg"
import cardJu from "./Ju.svg"
import cardQu from "./Qu.svg"
import cardKu from "./Ku.svg"

import carduC from "./uC.svg"
import carduH from "./uH.svg"
import carduS from "./uS.svg"
import carduD from "./uD.svg"

import carduu from "./uu.svg"

export const CARDS: Record<CardName, string> = {
    "AC": cardAC,
    "2C": card2C,
    "3C": card3C,
    "4C": card4C,
    "5C": card5C,
    "6C": card6C,
    "7C": card7C,
    "8C": card8C,
    "9C": card9C,
    "10C": card10C,
    "JC": cardJC,
    "QC": cardQC,
    "KC": cardKC,

    "AH": cardAH,
    "2H": card2H,
    "3H": card3H,
    "4H": card4H,
    "5H": card5H,
    "6H": card6H,
    "7H": card7H,
    "8H": card8H,
    "9H": card9H,
    "10H": card10H,
    "JH": cardJH,
    "QH": cardQH,
    "KH": cardKH,

    "AS": cardAS,
    "2S": card2S,
    "3S": card3S,
    "4S": card4S,
    "5S": card5S,
    "6S": card6S,
    "7S": card7S,
    "8S": card8S,
    "9S": card9S,
    "10S": card10S,
    "JS": cardJS,
    "QS": cardQS,
    "KS": cardKS,

    "AD": cardAD,
    "2D": card2D,
    "3D": card3D,
    "4D": card4D,
    "5D": card5D,
    "6D": card6D,
    "7D": card7D,
    "8D": card8D,
    "9D": card9D,
    "10D": card10D,
    "JD": cardJD,
    "QD": cardQD,
    "KD": cardKD,

    "Au": cardAu,
    "2u": card2u,
    "3u": card3u,
    "4u": card4u,
    "5u": card5u,
    "6u": card6u,
    "7u": card7u,
    "8u": card8u,
    "9u": card9u,
    "10u": card10u,
    "Ju": cardJu,
    "Qu": cardQu,
    "Ku": cardKu,

    "uC": carduC,
    "uH": carduH,
    "uS": carduS,
    "uD": carduD,

    "uu": carduu
}