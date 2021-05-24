#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

pub mod api;
pub mod pages;
pub mod templates;

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

use templates::register_functions;

use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;

fn main() {
    rocket::ignite()
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
        .mount("/static", StaticFiles::from("./static"))
        .launch();
}