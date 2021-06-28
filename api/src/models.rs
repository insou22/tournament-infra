#![allow(unused)]
use serde::{Serialize, Deserialize};

#[derive(Queryable, Serialize, Clone)]
pub struct User {
    #[serde(skip)]
    pub id: i32,
    pub username: String,
    pub display_name: String,
}

#[derive(Queryable)]
pub struct Ranking {
    pub id: i32,
    pub user_id: i32,
    pub tournament_id: i32,
    pub elo: i32,
}

#[derive(Queryable)]
pub struct Game {
    id: i32,
    user_id: i32,
    tournament_id: i32,
    created_at: Option<i32>,
    points: Option<i32>,
    elo_change: Option<i32>,
}

#[derive(Queryable, Serialize)]
pub struct Binary {
    #[serde(skip)]
    pub id: i32,
    #[serde(skip)]
    pub user_id: i32,
    #[serde(skip)]
    pub tournament_id: i32,
    pub created_at: i32,
    pub hash: String,
    pub time_taken_ms: Option<i32>,
    pub timed_out: Option<bool>,
}

#[derive(Queryable)]
pub struct Turn {
    id: i32,
    game_id: i32,
    turn_number: i32,
    player_id: i32,
    binary_id: i32,
    created_at: i32,
    time_taken_ms: Option<i32>,
    timed_out: Option<bool>,
    legal: Option<bool>,
    stdout: Option<String>,
    stderr: Option<String>,
    stdin: Option<String>,
}
