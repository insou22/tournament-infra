use std::collections::HashMap;

use rocket_contrib::templates::Template;

#[get("/signup")]
pub fn signup() -> Template {
    let context: HashMap<String, String> = HashMap::new();

    Template::render("signup", &context)
}
