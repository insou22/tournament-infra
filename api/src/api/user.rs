use crate::models::user::{User, UserInfo, UserProfile};
use rocket::http::CookieJar;
use rocket::response::status::{NotFound, Unauthorized};
use rocket::serde::json::Json;
use serde::Deserialize;

#[get("/userinfo")]
pub async fn get_userinfo(pool: &rocket::State<sqlx::SqlitePool>, user: User) -> Json<UserInfo> {
    Json(user.get_userinfo(pool.inner()).await)
}

#[get("/user")]
pub async fn get_current_user_profile(
    pool: &rocket::State<sqlx::SqlitePool>,
    user: User,
) -> Json<UserProfile> {
    Json(user.get_profile(1, pool.inner()).await)
}

#[get("/user/<username>")]
pub async fn get_user_profile(
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
