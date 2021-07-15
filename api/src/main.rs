#[macro_use] extern crate rocket;

pub mod api;
pub mod models;
pub mod cors;

use api::{
    login::*,
    logout::*,
    user::*,
    ranking::*,
    binary::*,
    game::*
};

#[launch]
async fn rocket() -> _ {
    let database_url = "sqlite://../test.db";
    let pool = sqlx::SqlitePool::connect(database_url)
        .await
        .expect("Failed to connect to database");

    rocket::build()
        .attach(cors::fairing())
        .manage(pool)
        .mount("/", routes![
            login,
            logout,
            get_userinfo,
            get_current_user_profile,
            get_user_profile,
            update_user_profile,
            get_rankings,
            get_current_user_binaries,
            get_user_binaries,
            get_current_user_binary,
            get_user_binary
        ])
}
