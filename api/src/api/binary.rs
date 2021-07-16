use crate::models::{
    binary::{Binary, BinaryResponse},
    user::User,
};
use rocket::serde::json::Json;

#[get("/user/<username>/binaries")]
pub async fn get_user_binaries(
    pool: &rocket::State<sqlx::SqlitePool>,
    username: &str,
) -> Option<Json<Vec<BinaryResponse>>> {
    match User::get_by_username(username, pool.inner()).await {
        Some(user) => Some(Json({
            // TODO: Ignore binaries that failed compilation.
            let binaries: Vec<Binary> = sqlx::query_as!(
                Binary,
                "SELECT * FROM binaries WHERE user_id=? AND tournament_id=? ORDER BY created_at DESC",
                user.id,
                1
            )
            .fetch_all(pool.inner())
            .await
            .expect("binary fetch all failed");

            let mut binaries_with_stats: Vec<BinaryResponse> = vec![];

            for binary in binaries {
                binaries_with_stats.push(BinaryResponse {
                    stats_summary: binary.get_stats_summary(pool.inner()).await,
                    binary,
                })
            }

            binaries_with_stats
        })),
        None => None,
    }
}

#[get("/user/<username>/binary/<hash>")]
pub async fn get_user_binary(
    pool: &rocket::State<sqlx::SqlitePool>,
    username: &str,
    hash: &str,
) -> Option<Json<BinaryResponse>> {
    // TODO: Return 404 for binaries that failed compilation if the logged in user is not the owner.
    match Binary::get_by_username_and_hash(username, hash, pool.inner()).await {
        Some(binary) => Some(Json(BinaryResponse {
            stats_summary: binary.get_stats_summary(pool.inner()).await,
            binary,
        })),
        None => None,
    }
}

// #[put("/binaries")]
