use crate::models::game::{Game, GameResponse};
use rocket::serde::json::Json;

#[get("/games")]
pub async fn get_games(pool: &rocket::State<sqlx::SqlitePool>) -> Json<Vec<GameResponse>> {
    let games: Vec<Game> = sqlx::query_as!(Game, "SELECT * FROM games ORDER BY completed_at DESC")
        .fetch_all(pool.inner())
        .await
        .expect("game fetch all failed");

    let mut games_with_players: Vec<GameResponse> = vec![];

    for game in games {
        games_with_players.push(GameResponse {
            // Sue Me: https://sqlite.org/np1queryprob.html
            players: game.get_players(pool.inner()).await,
            game,
            turns: None,
        })
    }

    Json(games_with_players)
}

// #[get("/user/<username>/games")]

// #[get("/user/<username>/binary/<hash>/games")]
