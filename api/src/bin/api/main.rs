#![allow(unused)]

#[macro_use]
extern crate rocket;

mod cors;
mod routes;

use routes::{binary::*, game::*, login::*, logout::*, ranking::*, user::*};

#[launch]
async fn rocket() -> _ {
    dotenv::dotenv().ok();

    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info,sqlx=warn"))
        .init();

    let celery_app = tournament_api::tasks::celery_app_factory().await.expect("Failed to construct celery app.");

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL environment variable should be set (via .env or otherwise).");
    let pool = sqlx::SqlitePool::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    rocket::build()
        .attach(cors::fairing())
        .attach(rocket::fairing::AdHoc::config::<
            tournament_api::config::Config,
        >())
        .manage(pool)
        .manage(celery_app)
        .mount(
            "/",
            routes![
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
            ],
        )
}
