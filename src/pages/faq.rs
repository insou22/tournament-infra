use std::collections::HashMap;

use rocket_contrib::templates::Template;

#[get("/faq")]
pub fn faq() -> Template {
    let context: HashMap<String, String> = HashMap::new();

    Template::render("faq", &context)
}
