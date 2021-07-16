use crate::models::{
    binary::Binary,
    game::{Game, GameResponse},
    user::User,
};
use rocket::serde::json::Json;

#[get("/games")]
pub async fn get_games(pool: &rocket::State<sqlx::SqlitePool>) -> Json<Vec<GameResponse>> {
    let games: Vec<Game> =
        sqlx::query_as!(Game, "SELECT * FROM games ORDER BY games.completed_at DESC")
            .fetch_all(pool.inner())
            .await
            .expect("game fetch all failed");

    let mut games_with_players: Vec<GameResponse> = vec![];

    for game in games {
        games_with_players.push(GameResponse {
            players: game.get_players(pool.inner()).await,
            game,
            turns: None,
        })
    }

    Json(games_with_players)
}

#[get("/user/<username>/games")]
pub async fn get_user_games(
    pool: &rocket::State<sqlx::SqlitePool>,
    username: &str,
) -> Option<Json<Vec<GameResponse>>> {
    let user = User::get_by_username(username, pool.inner()).await;

    if user.is_none() {
        return None;
    }

    let user = user.unwrap();

    let games: Vec<Game> = sqlx::query_as!(
        Game,
        "SELECT games.* FROM players JOIN games ON players.game_id=games.id WHERE players.user_id=? ORDER BY games.completed_at DESC",
        user.id
    )
    .fetch_all(pool.inner())
    .await
    .expect("user games fetch failed");

    let mut games_with_players: Vec<GameResponse> = vec![];

    for game in games {
        games_with_players.push(GameResponse {
            players: game.get_players(pool.inner()).await,
            game,
            turns: None,
        })
    }

    Some(Json(games_with_players))
}

#[get("/user/<username>/binary/<hash>/games")]
pub async fn get_binary_games(
    pool: &rocket::State<sqlx::SqlitePool>,
    username: &str,
    hash: &str,
) -> Option<Json<Vec<GameResponse>>> {
    let binary = Binary::get_by_username_and_hash(username, hash, pool.inner()).await;

    if binary.is_none() {
        return None;
    }

    let binary = binary.unwrap();

    let games: Vec<Game> = sqlx::query_as!(
        Game,
        "SELECT games.* FROM players JOIN games ON players.game_id=games.id WHERE players.binary_id=? ORDER BY games.completed_at DESC",
        binary.id
    )
    .fetch_all(pool.inner())
    .await
    .expect("user games fetch failed");

    let mut games_with_players: Vec<GameResponse> = vec![];

    for game in games {
        games_with_players.push(GameResponse {
            players: game.get_players(pool.inner()).await,
            game,
            turns: None,
        })
    }

    Some(Json(games_with_players))
}

#[get("/game/<id>")]
pub async fn get_game(
    pool: &rocket::State<sqlx::SqlitePool>,
    user: Option<User>,
    id: i64,
) -> Option<Json<GameResponse>> {
    let game = Game::get_by_id(id, pool.inner()).await;

    if game.is_none() {
        return None;
    }
    
    let game = game.unwrap();

    Some(Json(GameResponse {
        players: game.get_players(pool.inner()).await,
        turns: {
            let mut turns = game.get_turns(pool.inner()).await;

            for mut turn in &mut turns {
                if user.is_none() || user.as_ref().unwrap().id != turn.user_id {
                    turn.streams = None;
                }
            }

            Some(turns)
        },
        game
    }))
}
