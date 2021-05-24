#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

pub mod pages;
pub mod templates;

use pages::{
    index::*,
    rankings::*,
    spec::*,
    faq::*,
    about::*,
    login::*,
    signup::*,
};

use templates::register_functions;

use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;

fn main() {
    rocket::ignite()
        .attach(Template::custom(|engines| register_functions(&mut engines.tera)))
        .mount("/", routes![index, rankings, spec, faq, about, login, signup])
        .mount("/static", StaticFiles::from("./static"))
        .launch();
}