use tournament_api::paginate::{Cursor, Paginatable, Paginate, Paginated};
use rocket::http::Status;
use rocket::serde::{json::Json, Serialize};

#[derive(Serialize)]
pub struct RankingResponse {
    pub username: String,
    pub display_name: String,
    pub rating: i64,
}

impl Paginatable for RankingResponse {
    type CursorType = i64;
    fn get_cursor(&self) -> Self::CursorType {
        self.rating
    }
}

#[get("/rankings?<per_page>&<cursor>")]
pub async fn get_rankings(
    pool: &rocket::State<sqlx::SqlitePool>,
    config: &rocket::State<tournament_api::config::Config>,
    per_page: Option<i64>,
    cursor: Option<String>,
) -> Result<Json<Paginated<RankingResponse>>, Status> {
    let paginate = Paginate::new(per_page, cursor).or(Err(Status::BadRequest))?;

    let rankings: Vec<RankingResponse> = match paginate.cursor {
        Cursor::None => sqlx::query_as!(
            RankingResponse,
            "SELECT username, display_name, rating FROM rankings JOIN users ON rankings.user_id=users.id
            WHERE tournament_id=?
            ORDER BY rating DESC LIMIT ?",
            config.inner().current_tournament_id,
            paginate.per_page_with_cursor
        )
        .fetch_all(pool.inner())
        .await
        .expect("rankings fetch failed with no cursor"),
        Cursor::Next(c) => sqlx::query_as!(
            RankingResponse,
            "SELECT username, display_name, rating FROM rankings JOIN users ON rankings.user_id=users.id
            WHERE tournament_id=? AND rating<?
            ORDER BY rating DESC LIMIT ?",
            config.inner().current_tournament_id,
            c,
            paginate.per_page_with_cursor
        )
        .fetch_all(pool.inner())
        .await
        .expect("rankings fetch failed with next cursor"),
        Cursor::Prev(c) => sqlx::query_as!(
            RankingResponse,
            "SELECT username, display_name, rating FROM rankings JOIN users ON rankings.user_id=users.id
            WHERE tournament_id=? AND rating>?
            ORDER BY rating DESC LIMIT ?",
            config.inner().current_tournament_id,
            c,
            paginate.per_page_with_cursor
        )
        .fetch_all(pool.inner())
        .await
        .expect("rankings fetch failed with prev cursor"),
    };

    Ok(Json(Paginated::new(rankings, paginate)))
}
