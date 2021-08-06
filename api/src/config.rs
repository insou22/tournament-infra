#[derive(serde::Deserialize, Debug)]
pub struct CookieConfig {
    pub secure: bool,
    pub same_site: String
}

#[derive(serde::Deserialize, Debug)]
pub struct Config {
    pub current_tournament_id: i64,
    pub cookies: CookieConfig
}