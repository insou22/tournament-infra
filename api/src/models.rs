#![allow(unused)]

#[derive(Queryable)]
pub struct User {
    id: i32,
    username: String,
    display_name: String,
}

#[derive(Queryable)]
pub struct Ranking {
    id: i32,
    user_id: i32,
    tournament_id: i32,
    elo: i32,
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

#[derive(Queryable)]
pub struct Binary {
    id: i32,
    user_id: i32,
    tournament_id: i32,
    created_at: i32,
    time_taken_ms: Option<i32>,
    timed_out: Option<bool>,
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
