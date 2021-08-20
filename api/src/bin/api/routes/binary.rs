use ring::digest::{Context, SHA256};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::tokio::time::timeout;
use std::io::{BufReader, Read};
use tournament_api::models::{
    binary::{Binary, BinaryResponse},
    user::User,
};
use tournament_api::paginate::{Cursor, Paginate, Paginated};

#[get("/user/<username>/binaries?<per_page>&<cursor>")]
pub async fn get_user_binaries(
    pool: &rocket::State<sqlx::SqlitePool>,
    current_user: Option<User>,
    username: &str,
    config: &rocket::State<tournament_api::config::Config>,
    per_page: Option<i64>,
    cursor: Option<String>,
) -> Result<Json<Paginated<BinaryResponse>>, Status> {
    let mut conn = pool
        .inner()
        .acquire()
        .await
        .expect("could not acquire pool connection");

    match User::get_by_username(username, &mut conn).await.expect("could not fetch user") {
        Some(user) => Ok(Json({
            let paginate = Paginate::new(per_page, cursor).or(Err(Status::BadRequest))?;

            let binaries: Vec<Binary> = match paginate.cursor {
                Cursor::None => sqlx::query_as!(
                    Binary,
                    "SELECT * FROM binaries
                    WHERE user_id=? AND tournament_id=?
                    ORDER BY created_at DESC LIMIT ?",
                    user.id,
                    config.inner().current_tournament_id,
                    paginate.per_page_with_cursor
                )
                .fetch_all(&mut conn)
                .await
                .expect("binary fetch for user failed with no cursor"),
                Cursor::Next(c) => sqlx::query_as!(
                    Binary,
                    "SELECT * FROM binaries
                    WHERE user_id=? AND tournament_id=? AND created_at<?
                    ORDER BY created_at DESC LIMIT ?",
                    user.id,
                    config.inner().current_tournament_id,
                    c,
                    paginate.per_page_with_cursor
                )
                .fetch_all(&mut conn)
                .await
                .expect("binary fetch for user failed with next cursor"),
                Cursor::Prev(c) => sqlx::query_as!(
                    Binary,
                    "SELECT * FROM binaries
                    WHERE user_id=? AND tournament_id=? AND created_at>?
                    ORDER BY created_at DESC LIMIT ?",
                    user.id,
                    config.inner().current_tournament_id,
                    c,
                    paginate.per_page_with_cursor
                )
                .fetch_all(&mut conn)
                .await
                .expect("binary fetch for user failed with prev cursor"),
            };

            let mut binaries_with_stats: Vec<BinaryResponse> = vec![];

            for binary in binaries {
                if binary.compile_result == "success"
                    || current_user
                        .as_ref()
                        .filter(|cu| cu.id == user.id)
                        .is_some()
                {
                    binaries_with_stats.push(BinaryResponse {
                        stats_summary: binary.get_stats_summary(&mut conn).await.expect("could not fetch binary stats summary"),
                        binary,
                    })
                }
            }

            Paginated::new(binaries_with_stats, paginate)
        })),
        None => Err(Status::NotFound),
    }
}

#[get("/user/<username>/binary/<hash>")]
pub async fn get_user_binary(
    pool: &rocket::State<sqlx::SqlitePool>,
    current_user: Option<User>,
    username: &str,
    hash: &str,
) -> Option<Json<BinaryResponse>> {
    let mut conn = pool
        .inner()
        .acquire()
        .await
        .expect("could not acquire pool connection");
    match Binary::get_by_username_and_hash(username, hash, &mut conn).await.expect("could not fetch binary") {
        Some(binary) => {
            if binary.compile_result == "success"
                || current_user.filter(|cu| cu.username == username).is_some()
            {
                Some(Json(BinaryResponse {
                    stats_summary: binary
                        .get_stats_summary(&mut conn)
                        .await
                        .expect("could not fetch binary stats summary"),
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

#[put("/binaries", format = "multipart/form-data", data = "<file>")]
pub async fn put_user_binary(
    pool: &rocket::State<sqlx::SqlitePool>,
    celery_app: &rocket::State<std::sync::Arc<celery::Celery<celery::broker::RedisBroker>>>,
    current_user: User,
    config: &rocket::State<tournament_api::config::Config>,
    mut file: rocket::form::Form<rocket::fs::TempFile<'_>>,
) -> Json<BinaryResponse> {
    let mut conn = pool
        .inner()
        .acquire()
        .await
        .expect("could not acquire pool connection");

    // SQLite's max integer size is the same as i64, so we'll convert to that.
    let upload_time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64;
    let filename = format!("{}-{}", current_user.username, upload_time);
    let path =
        std::path::Path::new(&config.inner().code_upload_directory).join(format!("{}.c", filename));
    file.persist_to(&path).await.expect("file persist failed.");

    let hash = &hash_code(upload_time, &path)[..config.inner().code_hash_length];

    let binary_path = std::path::Path::new(&config.inner().binary_directory).join(filename);
    let compilation_future = async_process::Command::new("gcc")
        .arg(path.into_os_string())
        .arg("-o")
        .arg(binary_path)
        .kill_on_drop(true)
        .status();
    let compilation_status = match timeout(
        std::time::Duration::from_millis(config.inner().compilation_timeout),
        compilation_future,
    )
    .await
    {
        Ok(Ok(s)) => {
            if s.success() {
                "success"
            } else {
                "failed"
            }
        }
        Ok(Err(_)) => "failed",
        Err(_) => "timed_out",
    };

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
        compilation_status,
        current_user.id,
        hash
    )
    .fetch_one(&mut conn)
    .await
    .expect("binary insert into failed. note that there is now an orphaned C file in the uploads directory!");

    let other_binaries = sqlx::query!(
        r#"SELECT username, hash, MAX(created_at) AS "created_at!: i64" FROM binaries JOIN users ON binaries.user_id=users.id
        WHERE compile_result='success' AND user_id<>?
        GROUP BY user_id"#,
        current_user.id
    ).fetch_all(&mut conn).await.expect("binary get all for round robin failed.");

    for other_binary in other_binaries {
        celery_app
            .inner()
            .send_task(tournament_api::tasks::play::new(
                "round-1".to_owned(), // TODO: Don't hardcode this.
                vec![
                    (current_user.username.clone(), hash.to_owned()),
                    (other_binary.username.clone(), other_binary.hash),
                ],
            ))
            .await
            .expect("couldn't send task to consumer");
    }

    Json(BinaryResponse {
        stats_summary: binary
            .get_stats_summary(&mut conn)
            .await
            .expect("could not fetch binary stats summary"),
        binary,
    })
}
