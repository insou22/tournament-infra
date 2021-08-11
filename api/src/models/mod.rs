pub mod user;
pub mod binary;
pub mod game;

pub struct Ranking {
    pub id: i64,
    pub user_id: i64,
    pub tournament_id: i64,
    pub rating: i64,
}