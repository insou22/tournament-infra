use std::collections::HashMap;

use rocket::http::CookieJar;
use rocket_dyn_templates::Template;

#[get("/")]
pub fn index(cookies: &CookieJar<'_>) -> Template {
    let mut context: HashMap<String, String> = HashMap::new();
    
    if let Some(zid) = cookies.get_private("zid") {
        context.insert("zid".to_string(), zid.value().to_string());
    }

    Template::render("index", &context)
}
