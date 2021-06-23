#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;

pub mod api;
pub mod schema;
pub mod models;
pub mod cors;

use api::{
    login::*,
    logout::*,
    user::*
};

use rocket_sync_db_pools::database;

#[database("main")]
pub struct MainDbConn(diesel::SqliteConnection);

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(MainDbConn::fairing())
        .attach(cors::fairing())
        .mount("/", routes![
            login,
            logout,
            current_user_profile,
            //user_profile
        ])
}
