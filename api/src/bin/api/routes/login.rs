use rocket::{
    http::{Cookie, CookieJar, SameSite},
    serde::json::Json,
};
use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};
use tournament_api::models::user::{User, UserInfo};

#[derive(Deserialize)]
pub struct LoginRequest {
    zid: String,
    password: String,
}

#[derive(Serialize)]
#[serde(tag = "status")]
pub enum LoginResponse {
    Success(UserInfo),
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
    config: &rocket::State<tournament_api::config::Config>,
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
        let mut conn = pool
            .inner()
            .acquire()
            .await
            .expect("could not acquire pool connection");
        let mut cookie_builder = Cookie::build("zid", request.zid.to_string())
            .path("/")
            .expires(OffsetDateTime::now_utc() + Duration::weeks(6))
            .http_only(true);

        cookie_builder = match config.inner().cookies.same_site.as_str() {
            "none" => cookie_builder.same_site(SameSite::None),
            "lax" => cookie_builder.same_site(SameSite::Lax),
            "strict" => cookie_builder.same_site(SameSite::Strict),
            _ => cookie_builder,
        };

        if !config.inner().cookies.secure {
            cookie_builder = cookie_builder.secure(false);
        }

        let cookie = cookie_builder.finish();

        cookies.add_private(cookie);

        let zid = &request.zid;

        let user = match User::get_by_username(zid, &mut conn)
            .await
            .expect("could not fetch user")
        {
            Some(u) => u,
            None => {
                sqlx::query!(
                    "INSERT INTO users (username, display_name) VALUES (?, ?)",
                    zid,
                    zid
                )
                .execute(&mut conn)
                .await
                .expect("user insert failed");

                User::get_by_username(zid, &mut conn)
                    .await
                    .expect("could not fetch user")
                    .expect("just inserted user")
            }
        };

        Json(LoginResponse::Success(
            user.get_userinfo(config.inner().current_tournament_id, &mut conn)
                .await
                .expect("could not fetch userinfo"),
        ))
    } else {
        Json(LoginResponse::Failure(LoginFailure {
            message: "incorrect zID or password".to_string(),
        }))
    }
}
