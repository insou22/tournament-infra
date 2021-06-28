use crate::MainDbConn;
use crate::{
    models::{Binary, Ranking, User},
    schema::{binaries, games, rankings, turns, users},
};
use diesel::dsl::avg;
use diesel::prelude::*;
use rocket::http::CookieJar;
use rocket::response::status::{NotFound, Unauthorized};
use rocket::serde::json::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct UserInfoResponse {
    pub username: String,
    pub display_name: String,
    pub current_elo: Option<i32>,
}

#[get("/user")]
pub async fn user_info(
    conn: MainDbConn,
    cookies: &CookieJar<'_>,
) -> Result<Json<UserInfoResponse>, Unauthorized<()>> {
    match cookies.get_private("zid") {
        None => Err(Unauthorized(None)),
        Some(zid_cookie) => {
            let user = conn
                .run(move |c| {
                    users::table
                        .filter(users::columns::username.eq(zid_cookie.value()))
                        .first::<User>(c)
                        .expect("user find failed")
                })
                .await;
            let user_id = user.id;

            let current_ranking = conn
                .run(move |c| {
                    rankings::table
                        .filter(rankings::columns::user_id.eq(user_id))
                        .filter(
                            rankings::columns::tournament_id.eq(1i32), // TODO: Get this from the config file.
                        )
                        .first::<Ranking>(c)
                        .optional()
                        .expect("ranking find failed")
                })
                .await;
            let current_elo = current_ranking.and_then(|r| Some(r.elo));
            Ok(Json(UserInfoResponse {
                username: user.username,
                display_name: user.display_name,
                current_elo,
            }))
        }
    }
}

#[derive(Serialize)]
pub struct TournamentStats {
    pub ranking: u32,
    pub wins: u32,
    pub losses: u32,
    pub draws: u32,
    pub elo: u32,
    pub average_turn_run_time_ms: u32,
}

#[derive(Serialize)]
pub struct BinaryStats {
    pub wins: usize,
    pub losses: usize,
    pub draws: usize,
    pub win_loss: f32,
    pub win_loss_ratio_percentage_change: Option<f32>,
    pub average_turn_run_time_ms: usize,
    pub average_turn_run_time_ms_percentage_change: Option<f32>,
}

#[derive(Serialize)]
pub struct BinaryResponse {
    #[serde(flatten)]
    pub binary: Binary,
    pub stats_summary: BinaryStats,
}

#[derive(Serialize)]
pub struct UserProfileResponse {
    #[serde(flatten)]
    pub user: User,
    pub current_tournament_stats_summary: Option<TournamentStats>,
    pub current_binary: Option<BinaryResponse>,
}

#[get("/user/<username>")]
pub async fn user_profile(
    username: &str,
    conn: MainDbConn,
) -> Result<Json<UserProfileResponse>, NotFound<()>> {
    let username = username.to_owned();

    let user = conn
        .run(|c| {
            users::table
                .filter(users::columns::username.eq(username))
                .first::<User>(c)
                .optional()
                .expect("user find failed")
        })
        .await;

    if user.is_none() {
        return Err(NotFound(()));
    }

    let user = user.unwrap();

    let user_id = user.id;
    let tournament_stats = conn.run(move |c| {}).await;

    let user_id = user.id;
    let binary = conn
        .run(move |c| {
            binaries::table
                .filter(binaries::columns::user_id.eq(user_id))
                .filter(binaries::columns::tournament_id.eq(1)) // TODO: Make this read from somewhere for current tournment id.
                .order(binaries::columns::created_at.desc())
                .first::<Binary>(c)
                .optional()
                .expect("binary find failed")
        })
        .await;

    return Ok(Json(UserProfileResponse {
        user,
        current_tournament_stats_summary: None,
        current_binary: match binary {
            Some(binary) => {
                let binary_id = binary.id;
                Some(BinaryResponse {
                    binary,
                    stats_summary: conn
                        .run(move |c| {
                            let wins = games::table
                                .filter(games::columns::binary_id.eq(binary_id))
                                .filter(games::columns::points.eq(2))
                                .count()
                                .execute(c)
                                .expect("wins count failed");
                            let losses = games::table
                                .filter(games::columns::binary_id.eq(binary_id))
                                .filter(games::columns::points.eq(0))
                                .count()
                                .execute(c)
                                .expect("losses count failed");
                            BinaryStats {
                                wins,
                                losses,
                                win_loss: wins as f32 / losses as f32,
                                draws: games::table
                                    .filter(games::columns::binary_id.eq(binary_id))
                                    .filter(games::columns::points.eq(1))
                                    .count()
                                    .execute(c)
                                    .expect("losses count failed"),
                                average_turn_run_time_ms: turns::table
                                    .select(avg(turns::columns::time_taken_ms))
                                    .filter(turns::columns::binary_id.eq(binary_id))
                                    .execute(c)
                                    .expect("time taken average failed"),
                                // TODO: Write queries to retrieve these.
                                average_turn_run_time_ms_percentage_change: None,
                                win_loss_ratio_percentage_change: None,
                            }
                        })
                        .await,
                })
            }
            None => None,
        },
    }));
}
