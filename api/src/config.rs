#[derive(serde::Deserialize, Debug)]
pub struct CookieConfig {
    pub secure: bool,
    pub same_site: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct Config {
    pub current_tournament_id: i64,
    pub cookies: CookieConfig,
    pub code_upload_directory: String,
    pub code_hash_length: usize,
    pub webapp_url: String,
    pub compilation_timeout: u64,
    pub binary_directory: String
}
