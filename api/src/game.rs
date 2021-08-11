use serde::Serialize;

pub enum TurnResult {
    Legal,
    Illegal,
    Invalid,
}

pub trait Game {
    // Creates a new game with random starting state.
    fn new() -> Self;
    // Gets data and the binary that should receive said data for the upcoming turn.
    fn get_turn_data(&self) -> (String, u32);
    // Pass the binary's response to this function.
    // Returns whether the response is legal, illegal or invalid,
    // along with a human readable string that represents the action taken.
    fn respond(&self, action: String) -> (TurnResult, String);
}
