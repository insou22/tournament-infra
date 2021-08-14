use crate::errors::*;

pub struct Ranking {
    pub id: i64,
    pub user_id: i64,
    pub tournament_id: i64,
    pub rating_mu: f64,
    pub rating_sigma: f64,
}