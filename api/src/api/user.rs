use crate::models::user::{User, UserInfo, UserProfile};
use rocket::serde::json::Json;
use serde::Deserialize;

#[get("/userinfo")]
pub async fn get_userinfo(pool: &rocket::State<sqlx::SqlitePool>, user: User) -> Json<UserInfo> {
    Json(user.get_userinfo(pool.inner()).await)
}

#[get("/user/<username>")]
pub async fn get_user_profile(
    pool: &rocket::State<sqlx::SqlitePool>,
    username: &str,
) -> Option<Json<UserProfile>> {
    match User::get_by_username(username, pool.inner()).await {
        Some(user) => Some(Json(user.get_profile(1, pool.inner()).await)),
        None => None,
    }
}

#[derive(Deserialize)]
pub struct ProfileUpdate {
    pub display_name: String,
}

#[patch("/user", format = "json", data = "<profile_patch>")]
pub async fn update_user_profile(
    pool: &rocket::State<sqlx::SqlitePool>,
    user: User,
    profile_patch: Json<ProfileUpdate>,
) -> Json<UserProfile> {
    let user = sqlx::query_as!(
        User,
        "UPDATE users SET display_name=? WHERE id=?; SELECT * FROM users WHERE id=?",
        profile_patch.display_name,
        user.id,
        user.id
    )
    .fetch_one(pool.inner())
    .await
    .expect("profile update failed");

    let profile = user.get_profile(1, pool.inner()).await;

    Json(profile)
}
