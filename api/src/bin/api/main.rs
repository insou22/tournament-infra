#![allow(unused)]

#[macro_use] extern crate rocket;

mod routes;
mod cors;

use routes::{
    login::*,
    logout::*,
    user::*,
    ranking::*,
    binary::*,
    game::*
};

#[launch]
async fn rocket() -> _ {
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL environment variable should be set (via .env or otherwise).");
    let pool = sqlx::SqlitePool::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    rocket::build()
        .attach(cors::fairing())
        .attach(rocket::fairing::AdHoc::config::<tournament_api::config::Config>())
        .manage(pool)
        .mount("/", routes![
            login,
            logout,
            get_userinfo,
            get_user_profile,
            update_user_profile,
            get_rankings,
            get_user_binaries,
            get_user_binary,
            get_games,
            get_user_games,
            get_binary_games,
            get_game,
            put_user_binary
        ])
}
