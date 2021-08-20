use rocket::serde::json::Json;
use serde::Deserialize;
use tournament_api::models::user::{User, UserInfo, UserProfile};

#[get("/userinfo")]
pub async fn get_userinfo(
    pool: &rocket::State<sqlx::SqlitePool>,
    user: User,
    config: &rocket::State<tournament_api::config::Config>,
) -> Json<UserInfo> {
    let mut conn = pool.inner().acquire().await.expect("could not acquire pool connection");
    Json(
        user.get_userinfo(config.inner().current_tournament_id, &mut conn)
            .await.expect("could not fetch user info"),
    )
}

#[get("/user/<username>")]
pub async fn get_user_profile(
    pool: &rocket::State<sqlx::SqlitePool>,
    username: &str,
    config: &rocket::State<tournament_api::config::Config>,
) -> Option<Json<UserProfile>> {
    let mut conn = pool.inner().acquire().await.expect("could not acquire pool connection");
    match User::get_by_username(username, &mut conn).await.expect("could not fetch user") {
        Some(user) => Some(Json(
            user.get_profile(config.inner().current_tournament_id, &mut conn)
                .await.expect("could not fetch user profile"),
        )),
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
    config: &rocket::State<tournament_api::config::Config>,
) -> Json<UserProfile> {
    let mut conn = pool.inner().acquire().await.expect("could not acquire pool connection");

    let user = sqlx::query_as!(
        User,
        "UPDATE users SET display_name=? WHERE id=?; SELECT * FROM users WHERE id=?",
        profile_patch.display_name,
        user.id,
        user.id
    )
    .fetch_one(&mut conn)
    .await
    .expect("profile update failed");

    let profile = user
        .get_profile(config.inner().current_tournament_id, &mut conn)
        .await
        .expect("could not fetch user profile");

    Json(profile)
}
