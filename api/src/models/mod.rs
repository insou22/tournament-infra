pub mod user;
pub mod binary;
pub mod game;

pub struct Ranking {
    pub id: i64,
    pub user_id: i64,
    pub tournament_id: i64,
    pub rating: i64,
}

pub struct Turn {
    id: i64,
    game_id: i64,
    turn_number: i64,
    user_id: i64,
    binary_id: i64,
    created_at: i64,
    time_taken_ms: Option<i64>,
    timed_out: Option<bool>,
    legal: Option<bool>,
    stdout: Option<String>,
    stderr: Option<String>,
    stdin: Option<String>,
}
