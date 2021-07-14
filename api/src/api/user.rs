use crate::models;
use rocket::http::CookieJar;
use rocket::response::status::{NotFound, Unauthorized};
use rocket::serde::json::Json;
use serde::Serialize;
use rocket::tokio::try_join;

#[derive(Serialize)]
pub struct UserInfoResponse {
    pub username: String,
    pub display_name: String,
    pub current_elo: Option<i64>,
}

#[get("/userinfo")]
pub async fn user_info(
    pool: &rocket::State<sqlx::SqlitePool>,
    cookies: &CookieJar<'_>,
) -> Result<Json<UserInfoResponse>, Unauthorized<()>> {
    match cookies.get_private("zid") {
        None => Err(Unauthorized(None)),
        Some(zid_cookie) => {
            let zid = zid_cookie.value();

            let user = sqlx::query_as!(models::User, "SELECT * FROM users WHERE username=?", zid)
                .fetch_one(pool.inner())
                .await
                .expect("user fetch failed");

            let ranking = sqlx::query_as!(
                models::Ranking,
                "SELECT * FROM rankings WHERE user_id=? AND tournament_id=?",
                user.id,
                1
            )
            .fetch_optional(pool.inner())
            .await
            .expect("optional ranking fetch failed");

            let current_elo = ranking.and_then(|r| Some(r.elo));

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
    pub ranking: i32,
    pub wins: i32,
    pub losses: i32,
    pub draws: i32,
    pub win_loss: f64,
    pub elo: i64,
    pub average_turn_run_time_ms: f64,
}

#[derive(Serialize)]
pub struct BinaryStats {
    pub wins: i32,
    pub losses: i32,
    pub draws: i32,
    pub win_loss: f64,
    pub win_loss_ratio_percentage_change: Option<f64>,
    pub average_turn_run_time_ms: f64,
    pub average_turn_run_time_ms_percentage_change: Option<f64>,
}

#[derive(Serialize)]
pub struct BinaryResponse {
    #[serde(flatten)]
    pub binary: models::Binary,
    pub stats_summary: BinaryStats,
}

#[derive(Serialize)]
pub struct UserProfileResponse {
    #[serde(flatten)]
    pub user: models::User,
    pub current_tournament_stats_summary: Option<TournamentStats>,
    pub current_binary: Option<BinaryResponse>,
}

async fn get_tournament_stats_summary(
    user: &models::User,
    pool: &sqlx::SqlitePool,
) -> Option<TournamentStats> {
    match sqlx::query_as!(models::Ranking, "SELECT * FROM rankings WHERE user_id=? AND tournament_id=?", user.id, 1).fetch_optional(pool).await.expect("ranking fetch failed") {
        None => None,
        Some(ranking) => {
            let (position_record, wins_record, losses_record, draws_record, average_turn_run_time_ms_record) = try_join!(
                sqlx::query!("SELECT COUNT(*) + 1 AS result FROM rankings WHERE elo > ? AND tournament_id=?", ranking.elo, 1).fetch_one(pool),
                sqlx::query!("SELECT COUNT(*) AS result FROM games WHERE user_id=? AND tournament_id=? AND points=?", user.id, 1, 2).fetch_one(pool),
                sqlx::query!("SELECT COUNT(*) AS result FROM games WHERE user_id=? AND tournament_id=? AND points=?", user.id, 1, 1).fetch_one(pool),
                sqlx::query!("SELECT COUNT(*) AS result FROM games WHERE user_id=? AND tournament_id=? AND points=?", user.id, 1, 0).fetch_one(pool),
                sqlx::query!(r#"SELECT AVG(time_taken_ms) AS "result!: f64" FROM turns JOIN games ON turns.game_id=games.id WHERE turns.user_id=? AND games.tournament_id=?"#, user.id, 1).fetch_one(pool)
            ).expect("a tournament stats fetch failed");

            Some(TournamentStats {
                ranking: position_record.result,
                elo: ranking.elo,
                win_loss: wins_record.result as f64 / losses_record.result as f64,
                wins: wins_record.result,
                losses: losses_record.result,
                draws: draws_record.result,
                average_turn_run_time_ms: average_turn_run_time_ms_record.result,
            })
        }
    }
}

async fn get_binary_stats_summary(
    binary: &models::Binary,
    pool: &sqlx::SqlitePool,
) -> BinaryStats {
    // TODO: Condense first three of these into one query using GROUP BY.
    let (wins_record, losses_record, draws_record, average_turn_run_time_ms_record) = try_join!(
        sqlx::query!("SELECT COUNT(*) AS result FROM games WHERE binary_id=? AND tournament_id=? AND points=?", binary.id, 1, 2).fetch_one(pool),
        sqlx::query!("SELECT COUNT(*) AS result FROM games WHERE binary_id=? AND tournament_id=? AND points=?", binary.id, 1, 1).fetch_one(pool),
        sqlx::query!("SELECT COUNT(*) AS result FROM games WHERE binary_id=? AND tournament_id=? AND points=?", binary.id, 1, 0).fetch_one(pool),
        sqlx::query!(r#"SELECT AVG(time_taken_ms) AS "result!: f64" FROM turns JOIN games ON turns.game_id=games.id WHERE turns.binary_id=? AND games.tournament_id=?"#, binary.id, 1).fetch_one(pool)
    ).expect("a binary stats fetch failed");

    return BinaryStats {
        wins: wins_record.result,
        losses: losses_record.result,
        draws: draws_record.result,
        win_loss: wins_record.result as f64 / losses_record.result as f64,
        average_turn_run_time_ms: average_turn_run_time_ms_record.result,
        average_turn_run_time_ms_percentage_change: None, // TODO: Get change from previous binary.
        win_loss_ratio_percentage_change: None
    };
}

#[get("/user/<username>")]
pub async fn user_profile(
    pool: &rocket::State<sqlx::SqlitePool>,
    username: &str,
) -> Result<Json<UserProfileResponse>, NotFound<()>> {
    let user = sqlx::query_as!(
        models::User,
        "SELECT * FROM users WHERE username=?",
        username
    )
    .fetch_optional(pool.inner())
    .await
    .expect("optional user fetch failed");

    if user.is_none() {
        return Err(NotFound(()));
    }

    let user = user.unwrap();

    let binary = sqlx::query_as!(
        models::Binary,
        "SELECT * FROM binaries WHERE user_id=? AND tournament_id=? ORDER BY created_at DESC LIMIT 1",
        user.id,
        1
    )
    .fetch_optional(pool.inner())
    .await
    .expect("optional binary fetch failed");

    return Ok(Json(UserProfileResponse {
        current_tournament_stats_summary: get_tournament_stats_summary(&user, pool.inner()).await,
        current_binary: match binary {
            Some(b) => Some(BinaryResponse {
                stats_summary: get_binary_stats_summary(&b, pool.inner()).await,
                binary: b
            }),
            None => None,
        },
        user
    }));
}
