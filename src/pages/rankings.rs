
use std::collections::HashMap;

use rocket_contrib::templates::Template;

#[get("/rankings")]
pub fn rankings() -> Template {
    let context: HashMap<String, String> = HashMap::new();

    Template::render("rankings", &context)
}
