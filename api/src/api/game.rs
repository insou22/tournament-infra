use crate::models::{
    binary::Binary,
    game::{Game, GameResponse},
    user::User,
};
use crate::paginate::Paginate;
use rocket::serde::json::Json;

#[get("/games?<per_page>&<page>")]
pub async fn get_games(
    pool: &rocket::State<sqlx::SqlitePool>,
    config: &rocket::State<crate::config::Config>,
    per_page: Option<u32>,
    page: Option<u32>,
) -> Json<Vec<GameResponse>> {
    let paginate = Paginate::new(per_page, page);

    let games: Vec<Game> = sqlx::query_as!(
        Game,
        "SELECT *
        FROM games
        WHERE tournament_id=?
        ORDER BY games.completed_at DESC
        LIMIT ?
        OFFSET ?", // TODO: Replace offsets with better pagination once it gets laggy.
        config.inner().current_tournament_id,
        paginate.limit,
        paginate.offset
    )
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

#[get("/user/<username>/games?<per_page>&<page>")]
pub async fn get_user_games(
    pool: &rocket::State<sqlx::SqlitePool>,
    username: &str,
    config: &rocket::State<crate::config::Config>,
    per_page: Option<u32>,
    page: Option<u32>,
) -> Option<Json<Vec<GameResponse>>> {
    let user = User::get_by_username(username, pool.inner()).await;

    if user.is_none() {
        return None;
    }

    let user = user.unwrap();

    let paginate = Paginate::new(per_page, page);

    let games: Vec<Game> = sqlx::query_as!(
        Game,
        "SELECT games.*
        FROM players
        JOIN games ON players.game_id=games.id
        WHERE players.user_id=? AND games.tournament_id=?
        ORDER BY games.completed_at DESC
        LIMIT ?
        OFFSET ?",
        user.id,
        config.inner().current_tournament_id,
        paginate.limit,
        paginate.offset
    )
    .fetch_all(pool.inner())
    .await
    .expect("user games fetch failed");

    let mut games_with_players: Vec<GameResponse> = vec![];

    for game in games {
        let players = game.get_players(pool.inner()).await;
        games_with_players.push(GameResponse {
            players,
            game,
            turns: None,
        })
    }

    Some(Json(games_with_players))
}

#[get("/user/<username>/binary/<hash>/games?<per_page>&<page>")]
pub async fn get_binary_games(
    pool: &rocket::State<sqlx::SqlitePool>,
    current_user: Option<User>,
    username: &str,
    hash: &str,
    per_page: Option<u32>,
    page: Option<u32>,
) -> Option<Json<Vec<GameResponse>>> {
    let binary = Binary::get_by_username_and_hash(username, hash, pool.inner()).await;

    if binary.is_none() {
        return None;
    }

    let binary = binary.unwrap();

    if binary.compile_result != "success"
        && current_user.filter(|cu| cu.username == username).is_none()
    {
        return None;
    }

    let paginate = Paginate::new(per_page, page);

    let games: Vec<Game> = sqlx::query_as!(
        Game,
        "SELECT games.*
        FROM players
        JOIN games ON players.game_id=games.id
        WHERE players.binary_id=?
        ORDER BY games.completed_at DESC
        LIMIT ?
        OFFSET ?",
        binary.id,
        paginate.limit,
        paginate.offset
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
                if user.is_none() || user.as_ref().filter(|u| u.id == turn.user_id).is_none() {
                    turn.streams = None;
                    turn.state = None;
                }
            }

            Some(turns)
        },
        game,
    }))
}
