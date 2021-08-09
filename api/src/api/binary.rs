use crate::models::{
    binary::{Binary, BinaryResponse},
    user::User,
};
use crate::paginate::Paginate;
use ring::digest::{Context, SHA256};
use rocket::serde::json::Json;
use std::io::{BufReader, Read};

#[get("/user/<username>/binaries?<per_page>&<page>")]
pub async fn get_user_binaries(
    pool: &rocket::State<sqlx::SqlitePool>,
    current_user: Option<User>,
    username: &str,
    config: &rocket::State<crate::config::Config>,
    per_page: Option<u32>,
    page: Option<u32>,
) -> Option<Json<Vec<BinaryResponse>>> {
    match User::get_by_username(username, pool.inner()).await {
        Some(user) => Some(Json({
            let paginate = Paginate::new(per_page, page);

            let binaries: Vec<Binary> = sqlx::query_as!(
                Binary,
                "SELECT *
                FROM binaries
                WHERE user_id=? AND tournament_id=?
                ORDER BY created_at DESC
                LIMIT ?
                OFFSET ?",
                user.id,
                config.inner().current_tournament_id,
                paginate.limit,
                paginate.offset
            )
            .fetch_all(pool.inner())
            .await
            .expect("binary fetch all failed");

            let mut binaries_with_stats: Vec<BinaryResponse> = vec![];

            for binary in binaries {
                if binary.compile_result == "success"
                    || current_user
                        .as_ref()
                        .filter(|cu| cu.id == user.id)
                        .is_some()
                {
                    binaries_with_stats.push(BinaryResponse {
                        stats_summary: binary.get_stats_summary(pool.inner()).await,
                        binary,
                    })
                }
            }

            binaries_with_stats
        })),
        None => None,
    }
}

#[get("/user/<username>/binary/<hash>")]
pub async fn get_user_binary(
    pool: &rocket::State<sqlx::SqlitePool>,
    current_user: Option<User>,
    username: &str,
    hash: &str,
) -> Option<Json<BinaryResponse>> {
    match Binary::get_by_username_and_hash(username, hash, pool.inner()).await {
        Some(binary) => {
            if binary.compile_result == "success"
                || current_user.filter(|cu| cu.username == username).is_some()
            {
                Some(Json(BinaryResponse {
                    stats_summary: binary.get_stats_summary(pool.inner()).await,
                    binary,
                }))
            } else {
                None
            }
        }
        None => None,
    }
}

// Hashes the uploaded file's unix timestamp and content to create a unique hash for the binary.
fn hash_code(upload_time: i64, file_path: &std::path::Path) -> String {
    let mut context = Context::new(&SHA256);
    let input = std::fs::File::open(file_path).expect("file open for hashing failed");
    let mut reader = BufReader::new(input);
    let mut buffer = [0; 1024];

    context.update(&upload_time.to_be_bytes());

    loop {
        let count = reader.read(&mut buffer).expect("read failed");
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }

    data_encoding::HEXLOWER.encode(context.finish().as_ref())
}

#[put(
    "/binaries",
    format = "application/x-www-form-urlencoded",
    data = "<file>"
)]
pub async fn put_user_binary(
    pool: &rocket::State<sqlx::SqlitePool>,
    current_user: User,
    config: &rocket::State<crate::config::Config>,
    mut file: rocket::fs::TempFile<'_>,
) -> Json<BinaryResponse> {
    // SQLite's max integer size is the same as i64, so we'll convert to that.
    let upload_time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64;
    let filename = format!("{}-{}.c", current_user.username, upload_time);
    let path = std::path::Path::new(&config.inner().code_upload_directory).join(filename);
    file.persist_to(&path).await.expect("file persist failed.");

    let hash = &hash_code(upload_time, &path)[..config.inner().code_hash_length];

    let binary = sqlx::query_as!(
        Binary,
        // Note that we have two SQL queries here. RETURNING is a SQLite >= 3.35 feature, so instead we'll immediately query for the new binary.
        "INSERT INTO binaries (
            user_id, tournament_id, created_at, hash, compile_result
        ) VALUES (
            ?, ?, ?, ?, ?
        );
        SELECT * FROM binaries WHERE user_id=? AND hash=?",
        current_user.id,
        config.inner().current_tournament_id,
        upload_time,
        hash,
        "not_compiled",
        current_user.id,
        hash
    )
    .fetch_one(pool.inner())
    .await
    .expect("binary insert into failed. note that there is now an orphaned C file in the uploads directory!");

    Json(BinaryResponse {
        stats_summary: binary.get_stats_summary(pool.inner()).await,
        binary,
    })
}
