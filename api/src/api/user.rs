use crate::models::{
    user::{User, UserInfo, UserProfile},
    Ranking
};
use rocket::http::CookieJar;
use rocket::response::status::{NotFound, Unauthorized};
use rocket::serde::json::Json;
use serde::Deserialize;

#[get("/userinfo")]
pub async fn user_info(
    pool: &rocket::State<sqlx::SqlitePool>,
    cookies: &CookieJar<'_>,
) -> Result<Json<UserInfo>, Unauthorized<()>> {
    match cookies.get_private("zid") {
        None => Err(Unauthorized(None)),
        Some(zid_cookie) => {
            let zid = zid_cookie.value();

            let user = User::get_by_username(zid, pool.inner()).await;

            if user.is_none() {
                return Err(Unauthorized(None));
            }

            let user = user.unwrap();

            let ranking = sqlx::query_as!(
                Ranking,
                "SELECT * FROM rankings WHERE user_id=? AND tournament_id=?",
                user.id,
                1
            )
            .fetch_optional(pool.inner())
            .await
            .expect("optional ranking fetch failed");

            let current_elo = ranking.and_then(|r| Some(r.elo));

            Ok(Json(UserInfo {
                username: user.username,
                display_name: user.display_name,
                current_elo,
            }))
        }
    }
}

#[get("/user")]
pub async fn own_user_profile(
    pool: &rocket::State<sqlx::SqlitePool>,
    cookies: &CookieJar<'_>,
) -> Result<Json<UserProfile>, Unauthorized<()>> {
    match cookies.get_private("zid") {
        None => Err(Unauthorized(None)),
        Some(zid_cookie) => {
            let zid = zid_cookie.value();

            let user = User::get_by_username(zid, pool.inner()).await;

            if user.is_none() {
                return Err(Unauthorized(None));
            }

            let user = user.unwrap();

            return Ok(Json(user.get_profile(1, pool.inner()).await));
        }
    }
}

#[get("/user/<username>")]
pub async fn user_profile(
    pool: &rocket::State<sqlx::SqlitePool>,
    username: &str,
) -> Result<Json<UserProfile>, NotFound<()>> {
    let user = User::get_by_username(username, pool.inner()).await;

    if user.is_none() {
        return Err(NotFound(()));
    }

    let user = user.unwrap();

    return Ok(Json(user.get_profile(1, pool.inner()).await));
}

#[derive(Deserialize)]
pub struct ProfileUpdate {
    pub display_name: String,
}

#[patch("/user", format = "json", data = "<profile_patch>")]
pub async fn update_user_profile(
    pool: &rocket::State<sqlx::SqlitePool>,
    cookies: &CookieJar<'_>,
    profile_patch: Json<ProfileUpdate>,
) -> Result<Json<UserProfile>, Unauthorized<()>> {
    match cookies.get_private("zid") {
        None => Err(Unauthorized(None)),
        Some(zid_cookie) => {
            let zid = zid_cookie.value();

            let user = User::get_by_username(zid, pool.inner()).await;

            if user.is_none() {
                return Err(Unauthorized(None));
            }

            let user = user.unwrap();

            sqlx::query!(
                "UPDATE users SET display_name=? WHERE id=?",
                profile_patch.display_name,
                user.id
            )
            .execute(pool.inner())
            .await
            .expect("profile update failed");

            // TODO: Investigate why the update is not immediately reflected in the user profile select.
            let mut profile = user.get_profile(1, pool.inner()).await;
            profile.user.display_name = profile_patch.display_name.clone();

            return Ok(Json(profile));
        }
    }
}
