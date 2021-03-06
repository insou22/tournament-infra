use rocket::http::Status;
use rocket::serde::json::Json;
use tournament_api::models::{
    binary::Binary,
    game::{Game, GameResponse},
    user::User,
};
use tournament_api::paginate::{Cursor, Paginate, Paginated};

#[get("/games?<per_page>&<cursor>")]
pub async fn get_games(
    pool: &rocket::State<sqlx::SqlitePool>,
    config: &rocket::State<tournament_api::config::Config>,
    per_page: Option<i64>,
    cursor: Option<String>,
) -> Result<Json<Paginated<GameResponse>>, Status> {
    let mut conn = pool
        .inner()
        .acquire()
        .await
        .expect("could not acquire pool connection");

    let paginate = Paginate::new(per_page, cursor).or(Err(Status::BadRequest))?;

    let games: Vec<Game> = match paginate.cursor {
        Cursor::None => sqlx::query_as!(
            Game,
            "SELECT * FROM games
            WHERE tournament_id=?
            ORDER BY games.created_at DESC LIMIT ?",
            config.inner().current_tournament_id,
            paginate.per_page_with_cursor
        )
        .fetch_all(&mut conn)
        .await
        .expect("game fetch all failed with no cursor"),
        Cursor::Next(c) => sqlx::query_as!(
            Game,
            "SELECT * FROM games
            WHERE tournament_id=? AND created_at<?
            ORDER BY games.created_at DESC LIMIT ?",
            config.inner().current_tournament_id,
            c,
            paginate.per_page_with_cursor
        )
        .fetch_all(&mut conn)
        .await
        .expect("game fetch all failed with next cursor"),
        Cursor::Prev(c) => sqlx::query_as!(
            Game,
            "SELECT * FROM games
            WHERE tournament_id=? AND created_at>?
            ORDER BY games.created_at ASC LIMIT ?",
            config.inner().current_tournament_id,
            c,
            paginate.per_page_with_cursor
        )
        .fetch_all(&mut conn)
        .await
        .expect("game fetch all failed with prev cursor"),
    };

    let mut games_with_players: Vec<GameResponse> = vec![];

    for game in games {
        games_with_players.push(GameResponse {
            players: game
                .get_players(&mut conn)
                .await
                .expect("could not fetch players"),
            game,
            turns: None,
        })
    }

    Ok(Json(Paginated::new(games_with_players, paginate)))
}

#[get("/user/<username>/games?<per_page>&<cursor>")]
pub async fn get_user_games(
    pool: &rocket::State<sqlx::SqlitePool>,
    username: &str,
    config: &rocket::State<tournament_api::config::Config>,
    per_page: Option<i64>,
    cursor: Option<String>,
) -> Result<Json<Paginated<GameResponse>>, Status> {
    let mut conn = pool
        .inner()
        .acquire()
        .await
        .expect("could not acquire pool connection");

    let user = User::get_by_username(username, &mut conn)
        .await
        .expect("could not fetch user");
    if user.is_none() {
        return Err(Status::NotFound);
    }

    let user = user.unwrap();

    let paginate = Paginate::new(per_page, cursor).or(Err(Status::BadRequest))?;

    let games: Vec<Game> = match paginate.cursor {
        Cursor::None => sqlx::query_as!(
            Game,
            "SELECT games.* FROM players JOIN games ON players.game_id=games.id
            WHERE players.user_id=? AND games.tournament_id=?
            ORDER BY games.completed_at DESC LIMIT ?",
            user.id,
            config.inner().current_tournament_id,
            paginate.per_page_with_cursor
        )
        .fetch_all(&mut conn)
        .await
        .expect("game fetch for user failed with no cursor"),
        Cursor::Next(c) => sqlx::query_as!(
            Game,
            "SELECT games.* FROM players JOIN games ON players.game_id=games.id
            WHERE players.user_id=? AND games.tournament_id=? AND games.created_at<?
            ORDER BY games.completed_at DESC LIMIT ?",
            user.id,
            config.inner().current_tournament_id,
            c,
            paginate.per_page_with_cursor
        )
        .fetch_all(&mut conn)
        .await
        .expect("game fetch for user failed with next cursor"),
        Cursor::Prev(c) => sqlx::query_as!(
            Game,
            "SELECT games.* FROM players JOIN games ON players.game_id=games.id
            WHERE players.user_id=? AND games.tournament_id=? AND games.created_at>?
            ORDER BY games.completed_at ASC LIMIT ?",
            user.id,
            config.inner().current_tournament_id,
            c,
            paginate.per_page_with_cursor
        )
        .fetch_all(&mut conn)
        .await
        .expect("game fetch for user failed with prev cursor"),
    };

    let mut games_with_players: Vec<GameResponse> = vec![];

    for game in games {
        let players = game
            .get_players(&mut conn)
            .await
            .expect("could not fetch players");
        games_with_players.push(GameResponse {
            players,
            game,
            turns: None,
        })
    }

    Ok(Json(Paginated::new(games_with_players, paginate)))
}

#[get("/user/<username>/binary/<hash>/games?<per_page>&<cursor>")]
pub async fn get_binary_games(
    pool: &rocket::State<sqlx::SqlitePool>,
    current_user: Option<User>,
    username: &str,
    hash: &str,
    per_page: Option<i64>,
    cursor: Option<String>,
) -> Result<Json<Paginated<GameResponse>>, Status> {
    let mut conn = pool
        .inner()
        .acquire()
        .await
        .expect("could not acquire pool connection");
    let binary = Binary::get_by_username_and_hash(username, hash, &mut conn)
        .await
        .expect("could not fetch binary");

    if binary.is_none() {
        return Err(Status::NotFound);
    }

    let binary = binary.unwrap();

    if binary.compile_result != "success"
        && current_user.filter(|cu| cu.username == username).is_none()
    {
        return Err(Status::NotFound);
    }

    let paginate = Paginate::new(per_page, cursor).or(Err(Status::BadRequest))?;

    let games: Vec<Game> = match paginate.cursor {
        Cursor::None => sqlx::query_as!(
            Game,
            "SELECT games.* FROM players JOIN games ON players.game_id=games.id
            WHERE players.binary_id=?
            ORDER BY games.completed_at DESC LIMIT ?",
            binary.id,
            paginate.per_page_with_cursor
        )
        .fetch_all(&mut conn)
        .await
        .expect("game fetch for binary failed with no cursor"),
        Cursor::Next(c) => sqlx::query_as!(
            Game,
            "SELECT games.* FROM players JOIN games ON players.game_id=games.id
            WHERE players.binary_id=? AND games.created_at<?
            ORDER BY games.completed_at DESC LIMIT ?",
            binary.id,
            c,
            paginate.per_page_with_cursor
        )
        .fetch_all(&mut conn)
        .await
        .expect("game fetch for binary failed with next cursor"),
        Cursor::Prev(c) => sqlx::query_as!(
            Game,
            "SELECT games.* FROM players JOIN games ON players.game_id=games.id
            WHERE players.binary_id=? AND games.created_at>?
            ORDER BY games.completed_at ASC LIMIT ?",
            binary.id,
            c,
            paginate.per_page_with_cursor
        )
        .fetch_all(&mut conn)
        .await
        .expect("game fetch for binary failed with prev cursor"),
    };

    let mut games_with_players: Vec<GameResponse> = vec![];

    for game in games {
        games_with_players.push(GameResponse {
            players: game
                .get_players(&mut conn)
                .await
                .expect("could not fetch players"),
            game,
            turns: None,
        })
    }

    Ok(Json(Paginated::new(games_with_players, paginate)))
}

#[get("/game/<id>")]
pub async fn get_game(
    pool: &rocket::State<sqlx::SqlitePool>,
    user: Option<User>,
    id: i64,
) -> Option<Json<GameResponse>> {
    let mut conn = pool
        .inner()
        .acquire()
        .await
        .expect("could not acquire pool connection");

    let game = Game::get_by_id(id, &mut conn)
        .await
        .expect("could not fetch game");

    if game.is_none() {
        return None;
    }
    let game = game.unwrap();

    Some(Json(GameResponse {
        players: game
            .get_players(&mut conn)
            .await
            .expect("could not fetch players"),
        turns: {
            let mut turns = game
                .get_turns(&mut conn)
                .await
                .expect("could not fetch turns");

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
