use rocket::http::CookieJar;

#[post("/logout")]
pub fn logout(cookies: &CookieJar<'_>) {
    if let Some(zid) = cookies.get_private("zid") {
        cookies.remove_private(zid);
    }
}
