use std::collections::HashMap;

pub fn get_page_colours(args: &HashMap<String, tera::Value>) -> Result<tera::Value, rocket_dyn_templates::tera::Error> {
    let arg = args.get("page");

    let mut pages: HashMap<String, String> = HashMap::new();
    
    for page in ["home", "rankings", "spec", "faq", "about"] {
        pages.insert(page.to_string(), "text-white".to_string());

        if let Some(val) = arg {
            if val == page {
                pages.insert(page.to_string(), "text-secondary".to_string());
            }
        }
    }

    tera::to_value(pages)
        .map_err(|_err| "failed to serialiase pages".into())
}