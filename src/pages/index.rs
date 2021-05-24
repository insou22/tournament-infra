use std::collections::HashMap;

use rocket_contrib::templates::Template;

#[get("/")]
pub fn index() -> Template {
    let context: HashMap<String, String> = HashMap::new();

    Template::render("index", &context)
}
