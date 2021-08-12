pub mod round_1;
use crate::game::Game;
use std::collections::HashMap;

pub fn create_game_by_name(name: &str) -> Option<(impl Game, u8)> {
    match name {
        "round-1" => Some((round_1::Round1::new(), round_1::Round1::get_player_count())),
        _ => None
    }
}