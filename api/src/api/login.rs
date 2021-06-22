use crate::models::{Ranking, User};
use crate::schema::{rankings, users};
use crate::MainDbConn;
use diesel::prelude::*;
use rocket::{
    http::{Cookie, CookieJar},
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
    Success(LoginSuccess),
    Failure(LoginFailure),
}

#[derive(Serialize)]
pub struct LoginSuccess {
    username: String,
    display_name: String,
    current_elo: Option<i32>,
}

#[derive(Serialize)]
pub struct LoginFailure {
    message: String,
}

#[post("/login", data = "<request>")]
pub async fn login(
    conn: MainDbConn,
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
            .finish();

        cookies.add_private(cookie);

        let user = conn
            .run(move |c| {
                let user = users::table
                    .filter(users::columns::username.eq(&request.zid))
                    .first::<User>(c)
                    .optional()
                    .expect("user find failed");
                if user.is_none() {
                    diesel::insert_into(users::table)
                        .values((
                            users::columns::username.eq(&request.zid),
                            users::columns::display_name.eq(&request.zid),
                        ))
                        .execute(c)
                        .expect("insert into users table failed");
                    users::table
                        .filter(users::columns::username.eq(&request.zid))
                        .first::<User>(c)
                        .expect("user find failed")
                } else {
                    user.unwrap()
                }
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

        Json(LoginResponse::Success(LoginSuccess {
            username: user.username,
            display_name: user.display_name,
            current_elo
        }))
    } else {
        Json(LoginResponse::Failure(LoginFailure {
            message: "incorrect zID or password".to_string(),
        }))
    }
}
