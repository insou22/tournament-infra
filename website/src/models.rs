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
    created_at: i32,
    points: i32,
    elo_change: i32,
}

#[derive(Queryable)]
pub struct Binary {
    id: i32,
    user_id: i32,
    tournament_id: i32,
    created_at: i32,
    time_taken_ms: i32,
    timed_out: bool,
}

#[derive(Queryable)]
pub struct Turn {
    id: i32,
    game_id: i32,
    turn_number: i32,
    player_id: i32,
    binary_id: i32,
    created_at: i32,
    time_taken_ms: i32,
    timed_out: bool,
    legal: bool,
    stdout: String,
    stderr: String,
    stdin: String,
}
