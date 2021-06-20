use std::collections::HashMap;

use rocket_dyn_templates::Template;

#[get("/login")]
pub fn login() -> Template {
    let context: HashMap<String, String> = HashMap::new();

    Template::render("login", &context)
}
