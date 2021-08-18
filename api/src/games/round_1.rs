use super::common::Card;
use crate::errors::*;
use crate::game::{Game, GameState, TurnData, TurnResult};
use rand::prelude::*;
use rand::seq::SliceRandom;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Who {
    P1,
    P2,
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

    fn get_game_state(&self) -> GameState {
        let mut msg = format!(
            "{}\n",
            match self.turn {
                Who::P1 => 1,
                Who::P2 => 2,
            }
        );

        let hand = match self.turn {
            Who::P1 => &self.p1_hand,
            Who::P2 => &self.p2_hand,
        };

        if hand.len() == 0 {
            let mut p1_score = 0;
            let mut p2_score = 0;
            for trick in &self.prev_tricks {
                match trick.first {
                    Who::P1 => {
                        if trick.p1_card.suit != trick.p2_card.suit
                            || trick.p1_card.rank > trick.p2_card.rank
                        {
                            p1_score += 1;
                        } else {
                            p2_score += 1;
                        }
                    }
                    Who::P2 => {
                        if trick.p2_card.suit != trick.p1_card.suit
                            || trick.p2_card.rank > trick.p1_card.rank
                        {
                            p2_score += 1;
                        } else {
                            p1_score += 1;
                        }
                    }
                }
            }
            return GameState::Complete(vec![
                if p1_score > p2_score {
                    2
                } else if p1_score == p2_score {
                    1
                } else {
                    0
                },
                if p2_score > p1_score {
                    2
                } else if p2_score == p1_score {
                    1
                } else {
                    0
                },
            ]);
        }

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
                }
                Who::P2 => {
                    msg.push_str("2\n");
                    msg.push_str(&format!("{} {}\n", trick.p2_card.suit, trick.p2_card.rank));
                    msg.push_str(&format!("{} {}\n", trick.p1_card.suit, trick.p1_card.rank));
                }
            };
        }

        match self.curr {
            PartialTrick::First => {
                msg.push_str(&format!(
                    "{}\n",
                    match self.turn {
                        Who::P1 => 1,
                        Who::P2 => 2,
                    }
                ));
            }
            PartialTrick::Second(card) => {
                msg.push_str(&format!(
                    "{}\n",
                    match self.turn {
                        Who::P1 => 2,
                        Who::P2 => 1,
                    }
                ));
                msg.push_str(&format!("{} {}\n", card.suit, card.rank));
            }
        }

        GameState::Turn(TurnData {
            stdin: msg,
            player_index: match self.turn {
                Who::P1 => 0,
                Who::P2 => 1,
            },
        })
    }

    fn respond(&mut self, action: &str) -> (TurnResult, String) {
        // parse card
        let card = action.parse::<Card>();

        let mut tr = TurnResult::Legal;

        let mut hand = match self.turn {
            Who::P1 => &mut self.p1_hand,
            Who::P2 => &mut self.p2_hand,
        };

        if let Ok(card) = card {
            if !hand.contains(&card) {
                tr = TurnResult::Illegal;
            }
        } else {
            tr = TurnResult::Invalid;
        }

        let card_index = if tr == TurnResult::Legal {
            hand.iter()
                .position(|c| c == card.as_ref().unwrap())
                .expect("card not found")
        } else {
            let mut rng = rand::thread_rng();
            rng.gen_range(0..hand.len())
        };

        let card = hand.remove(card_index);

        match self.curr {
            PartialTrick::First => {
                self.curr = PartialTrick::Second(card);
                self.turn = match self.turn {
                    Who::P1 => Who::P2,
                    Who::P2 => Who::P1,
                };
            }
            PartialTrick::Second(first_card) => {
                self.curr = PartialTrick::First;

                match self.turn {
                    Who::P1 => {
                        self.prev_tricks.push(Trick {
                            first: Who::P2,
                            p1_card: card,
                            p2_card: first_card,
                        });

                        if first_card.suit != card.suit || first_card.rank > card.rank {
                            self.turn = Who::P2;
                        }
                    }
                    Who::P2 => {
                        self.prev_tricks.push(Trick {
                            first: Who::P1,
                            p1_card: first_card,
                            p2_card: card,
                        });

                        if first_card.suit != card.suit || first_card.rank > card.rank {
                            self.turn = Who::P1;
                        }
                    }
                };
            }
        };

        (tr, format!("{}", card))
    }

    fn get_player_count() -> usize {
        2
    }
}
