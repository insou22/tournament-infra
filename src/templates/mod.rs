pub mod functions;

use self::functions::get_page_colours;
use rocket_dyn_templates::tera::Tera;

pub fn register_functions(tera: &mut Tera) {
    tera.register_function("get_page_colours", get_page_colours);
}
