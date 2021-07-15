use serde::Serialize;
use rocket::tokio::try_join;

#[derive(Serialize)]
pub struct BinaryStats {
    pub wins: i32,
    pub losses: i32,
    pub draws: i32,
    pub win_loss: f64,
    pub win_loss_ratio_percentage_change: Option<f64>,
    pub average_turn_run_time_ms: f64,
    pub average_turn_run_time_ms_percentage_change: Option<f64>,
}

#[derive(Serialize)]
pub struct BinaryWithStats {
    #[serde(flatten)]
    pub binary: Binary,
    pub stats_summary: BinaryStats,
}

// TODO: Setup compile_result. Probably by adding an impl for sqlx::FromRow.
#[derive(Serialize)]
pub struct Binary {
    #[serde(skip)]
    pub id: i64,
    #[serde(skip)]
    pub user_id: i64,
    #[serde(skip)]
    pub tournament_id: i64,
    pub created_at: i64,
    pub hash: String,
    pub time_taken_ms: Option<i64>,
    pub timed_out: Option<bool>,
}

impl Binary {
    pub async fn get_by_username_and_hash(username: &str, hash: &str, pool: &sqlx::SqlitePool) -> Option<Self> {
        sqlx::query_as!(Self, "SELECT binaries.* FROM binaries INNER JOIN users ON users.id=binaries.user_id WHERE users.username=? AND hash=?", username, hash)
            .fetch_optional(pool)
            .await
            .expect("optional binary fetch by username and hash failed")
    }

    pub async fn get_stats_summary(&self, tournament_id: i64, pool: &sqlx::SqlitePool) -> BinaryStats {
        // TODO: Condense first three of these into one query using GROUP BY.
        let (wins_record, losses_record, draws_record, average_turn_run_time_ms_record) = try_join!(
            sqlx::query!("SELECT COUNT(*) AS result FROM games WHERE binary_id=? AND tournament_id=? AND points=?", self.id, tournament_id, 2).fetch_one(pool),
            sqlx::query!("SELECT COUNT(*) AS result FROM games WHERE binary_id=? AND tournament_id=? AND points=?", self.id, tournament_id, 1).fetch_one(pool),
            sqlx::query!("SELECT COUNT(*) AS result FROM games WHERE binary_id=? AND tournament_id=? AND points=?", self.id, tournament_id, 0).fetch_one(pool),
            sqlx::query!(r#"SELECT AVG(time_taken_ms) AS "result!: f64" FROM turns JOIN games ON turns.game_id=games.id WHERE turns.binary_id=? AND games.tournament_id=?"#, self.id, tournament_id).fetch_one(pool)
        ).expect("a binary stats fetch failed");

        return BinaryStats {
            wins: wins_record.result,
            losses: losses_record.result,
            draws: draws_record.result,
            win_loss: wins_record.result as f64 / losses_record.result as f64,
            average_turn_run_time_ms: average_turn_run_time_ms_record.result,
            average_turn_run_time_ms_percentage_change: None, // TODO: Get change from previous binary.
            win_loss_ratio_percentage_change: None,
        };
    }
}
