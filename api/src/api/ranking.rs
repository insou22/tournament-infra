use rocket::serde::{json::Json, Serialize};

#[derive(Serialize)]
pub struct RankingResponse {
    pub username: String,
    pub display_name: String,
    pub rating: i64,
}

#[get("/rankings")]
pub async fn get_rankings(pool: &rocket::State<sqlx::SqlitePool>) -> Json<Vec<RankingResponse>> {
    // TODO: Implement pagination.
    Json(sqlx::query_as!(
        RankingResponse,
        "SELECT username, display_name, rating FROM rankings INNER JOIN users ON rankings.user_id=users.id WHERE tournament_id=? ORDER BY rating DESC",
        1
    ).fetch_all(pool.inner()).await.expect("rankings fetch failed"))
}