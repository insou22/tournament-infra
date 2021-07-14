#[macro_use] extern crate rocket;

pub mod api;
pub mod models;
pub mod cors;

use api::{
    login::*,
    logout::*,
    user::*
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
            user_info,
            user_profile
        ])
}
