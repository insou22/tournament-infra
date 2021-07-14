#![allow(unused)]
use serde::{Serialize, Deserialize};

#[derive(Serialize, Clone)]
pub struct User {
    #[serde(skip)]
    pub id: i64,
    pub username: String,
    pub display_name: String,
}

pub struct Ranking {
    pub id: i64,
    pub user_id: i64,
    pub tournament_id: i64,
    pub elo: i64,
}

pub struct Game {
    id: i64,
    user_id: i64,
    tournament_id: i64,
    created_at: Option<i64>,
    points: Option<i64>,
    elo_change: Option<i64>,
}

#[derive(Serialize)]
pub struct Binary {
    #[serde(skip)]
    pub id: i64,
    #[serde(skip)]
    pub user_id: i64,
    #[serde(skip)]
    pub tournament_id: i64,
    pub created_at: i64,
    pub hash: String,
    pub time_taken_ms: Option<i64>,
    pub timed_out: Option<bool>,
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
