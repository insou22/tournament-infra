use std::collections::HashMap;

use rocket_contrib::templates::Template;

#[get("/spec")]
pub fn spec() -> Template {
    let context: HashMap<String, String> = HashMap::new();

    Template::render("spec", &context)
}
