use crate::api::user::UserInfoResponse;
use crate::models;
use rocket::{
    http::{Cookie, CookieJar, SameSite},
    serde::json::Json,
};
use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};

#[derive(Deserialize)]
pub struct LoginRequest {
    zid: String,
    password: String,
}

#[derive(Serialize)]
#[serde(tag = "status")]
pub enum LoginResponse {
    Success(UserInfoResponse),
    Failure(LoginFailure),
}

#[derive(Serialize)]
pub struct LoginFailure {
    message: String,
}

#[post("/login", data = "<request>")]
pub async fn login(
    pool: &rocket::State<sqlx::SqlitePool>,
    request: Json<LoginRequest>,
    cookies: &CookieJar<'_>,
) -> Json<LoginResponse> {
    let client = reqwest::Client::new();
    let params = [("zid", &request.zid), ("password", &request.password)];
    let res = client
        .post("https://cgi.cse.unsw.edu.au/~z5257261/zidauth.cgi")
        .form(&params)
        .send()
        .await;

    if let Err(err) = res {
        println!("Unexpected error: {}", err.to_string());
        return Json(LoginResponse::Failure(LoginFailure {
            message: "failed to connect to UNSW services".to_string(),
        }));
    }

    let response = res.expect("just checked error case and returned above");
    let status_body = response.text().await.expect("should never fail");
    let status = status_body.trim();

    if status == "true" {
        let cookie = Cookie::build("zid", request.zid.to_string())
            .path("/")
            // .secure(true)
            .http_only(true)
            .expires(OffsetDateTime::now_utc() + Duration::weeks(6))
            .same_site(SameSite::None) // TODO: Find a solution for this being needed.
            .finish();

        cookies.add_private(cookie);

        let zid = &request.zid;

        let user = match sqlx::query_as!(models::User, "SELECT * FROM users WHERE username=?", zid)
            .fetch_optional(pool.inner())
            .await
            .expect("optional user fetch failed")
        {
            Some(u) => u,
            None => {
                sqlx::query!(
                    "INSERT INTO users (username, display_name) VALUES (?, ?)",
                    zid,
                    zid
                )
                .execute(pool.inner())
                .await
                .expect("user insert failed");

                sqlx::query_as!(models::User, "SELECT * FROM users WHERE username=?", zid)
                    .fetch_one(pool.inner())
                    .await
                    .expect("user fetch failed")
            }
        };

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

        Json(LoginResponse::Success(UserInfoResponse {
            username: user.username,
            display_name: user.display_name,
            current_elo,
        }))
    } else {
        Json(LoginResponse::Failure(LoginFailure {
            message: "incorrect zID or password".to_string(),
        }))
    }
}
