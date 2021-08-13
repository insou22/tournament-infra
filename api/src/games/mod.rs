pub mod common;
pub mod round_1;
use crate::game::Game;
use std::collections::HashMap;

pub fn create_game_by_name(name: &str) -> Option<(impl Game, usize, i64)> {
    match name {
        "round-1" => Some((round_1::Round1::new(), round_1::Round1::get_player_count(), 1)),
        _ => None,
    }
}
