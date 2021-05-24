use rocket::http::Cookies;

#[post("/logout")]
pub fn logout_post(mut cookies: Cookies) {
    if let Some(zid) = cookies.get_private("zid") {
        cookies.remove_private(zid);
    }
}
