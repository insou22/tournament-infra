use std::collections::HashMap;

use rocket_contrib::templates::Template;

#[get("/about")]
pub fn about() -> Template {
    let context: HashMap<String, String> = HashMap::new();

    Template::render("about", &context)
}
