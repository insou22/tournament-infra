use crate::models::{
    binary::{Binary, BinaryWithStats},
    user::User,
};
use rocket::response::status::NotFound;
use rocket::serde::json::Json;

async fn fetch_user_binaries(user: &User, pool: &sqlx::SqlitePool) -> Vec<Binary> {
    sqlx::query_as!(
        Binary,
        "SELECT * FROM binaries WHERE user_id=? AND tournament_id=? ORDER BY created_at DESC",
        user.id,
        1
    )
    .fetch_all(pool)
    .await
    .expect("binary fetch all failed")
}

async fn attach_stats_to_binary_vec(
    binaries: Vec<Binary>,
    tournament_id: i64,
    pool: &sqlx::SqlitePool,
) -> Vec<BinaryWithStats> {
    let mut binaries_with_stats: Vec<BinaryWithStats> = vec![];

    for binary in binaries {
        binaries_with_stats.push(BinaryWithStats {
            stats_summary: binary.get_stats_summary(tournament_id, pool).await,
            binary,
        })
    }
    return binaries_with_stats;
}

#[get("/binaries")]
pub async fn get_current_user_binaries(
    pool: &rocket::State<sqlx::SqlitePool>,
    user: User,
) -> Json<Vec<BinaryWithStats>> {
    Json(
        attach_stats_to_binary_vec(
            fetch_user_binaries(&user, pool.inner()).await,
            1,
            pool.inner(),
        )
        .await,
    )
}

#[get("/user/<username>/binaries")]
pub async fn get_user_binaries(
    pool: &rocket::State<sqlx::SqlitePool>,
    username: &str,
) -> Result<Json<Vec<BinaryWithStats>>, NotFound<()>> {
    match User::get_by_username(username, pool.inner()).await {
        Some(user) => Ok(Json(
            attach_stats_to_binary_vec(
                fetch_user_binaries(&user, pool.inner()).await,
                1,
                pool.inner(),
            )
            .await,
        )),
        None => Err(NotFound(())),
    }
}

#[get("/binary/<hash>")]
pub async fn get_current_user_binary(
    pool: &rocket::State<sqlx::SqlitePool>,
    user: User,
    hash: &str,
) -> Result<Json<BinaryWithStats>, NotFound<()>> {
    match Binary::get_by_username_and_hash(&user.username, hash, pool.inner()).await {
        Some(binary) => Ok(Json(BinaryWithStats {
            stats_summary: binary.get_stats_summary(1, pool.inner()).await,
            binary,
        })),
        None => Err(NotFound(())),
    }
}

#[get("/user/<username>/binary/<hash>")]
pub async fn get_user_binary(
    pool: &rocket::State<sqlx::SqlitePool>,
    username: &str,
    hash: &str,
) -> Result<Json<BinaryWithStats>, NotFound<()>> {
    match Binary::get_by_username_and_hash(username, hash, pool.inner()).await {
        Some(binary) => Ok(Json(BinaryWithStats {
            stats_summary: binary.get_stats_summary(1, pool.inner()).await,
            binary,
        })),
        None => Err(NotFound(())),
    }
}

// #[put("/binaries")]
