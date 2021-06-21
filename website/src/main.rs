#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;

pub mod api;
pub mod pages;
pub mod templates;
pub mod schema;
pub mod models;

use api::{
    login::*,
    logout::*,
};

use pages::{
    index::*,
    rankings::*,
    spec::*,
    faq::*,
    about::*,
    login::*,
};

use rocket::{
    fs::FileServer,
};

use templates::register_functions;
use rocket_dyn_templates::Template;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::custom(|engines| register_functions(&mut engines.tera)))
        .mount("/", routes![
            index,
            rankings,
            spec,
            faq,
            about,
            login,
            login_post,
            logout_post,
        ])
        .mount("/static", FileServer::from("./static"))
}
