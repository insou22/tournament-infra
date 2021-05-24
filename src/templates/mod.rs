pub mod functions;

use self::functions::{
    get_page_colours,
};
use rocket_contrib::templates::tera::Tera;


pub fn register_functions(tera: &mut Tera) {
    tera.register_function("get_page_colours", Box::new(get_page_colours));
}
