use rocket::{http::{Cookie, CookieJar}, serde::json::Json};
use serde::{Deserialize, Serialize};
use time::{OffsetDateTime, Duration};

#[derive(Deserialize)]
pub struct LoginRequest {
    zid: String,
    password: String,
}

#[derive(Serialize)]
#[serde(tag = "status")]
pub enum LoginResponse {
    Success,
    Failure(LoginFailure),
}
#[derive(Serialize)]
pub struct LoginFailure {
    message: String,
}

#[post("/login", data = "<request>")]
pub fn login_post(request: Json<LoginRequest>, cookies: &CookieJar<'_>) -> Json<LoginResponse> {
    let client = reqwest::blocking::Client::new();
    let params = [("zid", &request.zid), ("password", &request.password)];
    let res = client.post("https://cgi.cse.unsw.edu.au/~z5257261/zidauth.cgi")
        .form(&params)
        .send();

    if let Err(err) = res {
        println!("Unexpected error: {}", err.to_string());
        return Json(
            LoginResponse::Failure(
                LoginFailure {
                    message: "failed to connect to UNSW services".to_string(),
                }
            )
        );
    }

    let response = res.expect("just checked error case and returned above");
    let status_body = response.text()
        .expect("should never fail");
    let status = status_body.trim();

    if status == "true" {
        let cookie = Cookie::build("zid", request.zid.to_string())
            .path("/")
            // .secure(true)
            .http_only(true)
            .expires(OffsetDateTime::now_utc() + Duration::weeks(6))
            .finish();

        cookies.add_private(cookie);
        
        Json(
            LoginResponse::Success
        )
    } else {
        Json(
            LoginResponse::Failure(
                LoginFailure {
                    message: "incorrect zID or password".to_string(),
                }
            )
        )
    }
}
