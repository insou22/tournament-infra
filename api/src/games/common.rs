use crate::errors::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Card {
    pub rank: u32,
    pub suit: u32,
}

impl std::str::FromStr for Card {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        if s.len() == 3 && &s[1..2] == " " {
            let suit = s[0..1].parse::<u32>()?;
            let rank = s[2..3].parse::<u32>()?;

            Ok(Self { suit, rank })
        } else {
            bail!("failed to parse card")
        }
    }
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} of {}", match self.rank {
            1 => "Ace".to_owned(),
            11 => "Jack".to_owned(),
            12 => "Queen".to_owned(),
            13 => "King".to_owned(),
            x => format!("{}", x)
        }, match self.suit {
            1 => "Diamonds",
            2 => "Hearts",
            3 => "Spades",
            4 => "Clubs",
            _ => "Invalid Suit"
        })
    }
}