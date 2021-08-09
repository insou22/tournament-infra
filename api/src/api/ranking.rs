use crate::paginate::Paginate;
use rocket::serde::{json::Json, Serialize};

#[derive(Serialize)]
pub struct RankingResponse {
    pub username: String,
    pub display_name: String,
    pub rating: i64,
}

#[get("/rankings?<per_page>&<page>")]
pub async fn get_rankings(
    pool: &rocket::State<sqlx::SqlitePool>,
    config: &rocket::State<crate::config::Config>,
    per_page: Option<u32>,
    page: Option<u32>,
) -> Json<Vec<RankingResponse>> {
    let paginate = Paginate::new(per_page, page);

    Json(
        sqlx::query_as!(
            RankingResponse,
            "SELECT username, display_name, rating
            FROM rankings
            INNER JOIN users ON rankings.user_id=users.id
            WHERE tournament_id=?
            ORDER BY rating DESC
            LIMIT ?
            OFFSET ?",
            config.inner().current_tournament_id,
            paginate.limit,
            paginate.offset
        )
        .fetch_all(pool.inner())
        .await
        .expect("rankings fetch failed"),
    )
}
