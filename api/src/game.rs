use serde::Serialize;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum TurnResult {
    Legal,
    Illegal,
    Invalid,
}

pub struct TurnData {
    pub stdin: String,
    pub player_index: usize,
}

pub enum GameState {
    Turn(TurnData),
    Complete(Vec<u32>),
}

pub trait Game {
    // Creates a new game with random starting state.
    fn new() -> Self;

    fn get_game_state(&self) -> GameState;

    // Pass the binary's response to this function.
    // Returns whether the response is legal, illegal or invalid,
    // along with a human readable string that represents the action taken.
    fn respond(&mut self, action: &str) -> (TurnResult, String);

    fn get_player_count() -> usize;
}
