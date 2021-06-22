#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;

pub mod api;
pub mod schema;
pub mod models;

use api::{
    login::*,
    logout::*,
};

use rocket_sync_db_pools::database;

#[database("main")]
pub struct MainDbConn(diesel::SqliteConnection);

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(MainDbConn::fairing())
        .mount("/", routes![
            login,
            logout,
        ])
}
