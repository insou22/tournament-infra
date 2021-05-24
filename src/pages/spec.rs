use std::collections::HashMap;

use rocket::http::Cookies;
use rocket_contrib::templates::Template;

#[get("/spec")]
pub fn spec(mut cookies: Cookies) -> Template {
    let mut context: HashMap<String, String> = HashMap::new();
    
    if let Some(zid) = cookies.get_private("zid") {
        context.insert("zid".to_string(), zid.value().to_string());
    }

    Template::render("spec", &context)
}
