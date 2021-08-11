use crate::game::{Game, TurnResult};
use rand::seq::SliceRandom;

#[derive(Clone, Copy)]
enum Who {
    P1,
    P2,
}

#[derive(Clone, Copy)]
struct Card {
    rank: u32,
    suit: u32,
}

#[derive(Clone, Copy)]
struct Trick {
    first: Who,
    p1_card: Card,
    p2_card: Card,
}

#[derive(Clone, Copy)]
enum PartialTrick {
    First,
    Second(Card),
}

pub struct Round1 {
    p1_hand: Vec<Card>,
    p2_hand: Vec<Card>,
    prev_tricks: Vec<Trick>,
    turn: Who,
    curr: PartialTrick,
}

impl Game for Round1 {
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mut cards = vec![];

        for rank in 1..6 {
            for suit in 1..5 {
                cards.push(Card { rank, suit });
            }
        }

        cards.shuffle(&mut rng);

        let (p1_hand, p2_hand) = cards.split_at(cards.len() / 2);
        let (p1_hand, p2_hand): (Vec<Card>, Vec<Card>) = (
            p1_hand.iter().copied().collect(),
            p2_hand.iter().copied().collect(),
        );

        let turn = if rand::random() { Who::P1 } else { Who::P2 };

        Self {
            p1_hand,
            p2_hand,
            prev_tricks: vec![],
            turn,
            curr: PartialTrick::First,
        }
    }

    fn get_turn_data(&self) -> (String, u32) {
        let mut msg = format!("{}\n", match self.turn {
            Who::P1 => 1,
            Who::P2 => 2
        });

        let hand = match self.turn {
            Who::P1 => &self.p1_hand,
            Who::P2 => &self.p2_hand
        };

        msg.push_str(&format!("{}\n", hand.len()));

        for card in hand {
            msg.push_str(&format!("{} {}\n", card.suit, card.rank));
        }

        msg.push_str(&format!("{}\n", self.prev_tricks.len()));

        for trick in &self.prev_tricks {
            match trick.first {
                Who::P1 => {
                    msg.push_str("1\n");
                    msg.push_str(&format!("{} {}\n", trick.p1_card.suit, trick.p1_card.rank));
                    msg.push_str(&format!("{} {}\n", trick.p2_card.suit, trick.p2_card.rank));
                },
                Who::P2 => {
                    msg.push_str("2\n");
                    msg.push_str(&format!("{} {}\n", trick.p2_card.suit, trick.p2_card.rank));
                    msg.push_str(&format!("{} {}\n", trick.p1_card.suit, trick.p1_card.rank));
                }
            };
        }

        match self.curr {
            PartialTrick::First => {
                msg.push_str(&format!("{}\n", match self.turn {
                    Who::P1 => 1,
                    Who::P2 => 2
                }));
            },
            PartialTrick::Second(card) => {
                msg.push_str(&format!("{}\n", match self.turn {
                    Who::P1 => 2,
                    Who::P2 => 1
                }));
                msg.push_str(&format!("{} {}\n", card.suit, card.rank));
            }
        }

        (msg, match self.turn {
            Who::P1 => 0,
            Who::P2 => 1
        })
    }

    fn respond(&self, action: String) -> (TurnResult, String) {
        (TurnResult::Invalid, "".to_owned())
    }
}
