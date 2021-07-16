use rocket::tokio::try_join;
use serde::Serialize;

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
pub struct BinaryResponse {
    #[serde(flatten)]
    pub binary: Binary,
    pub stats_summary: BinaryStats,
}

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
    pub compile_result: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compile_time_ms: Option<i64>,
}

impl Binary {
    pub async fn get_by_username_and_hash(
        username: &str,
        hash: &str,
        pool: &sqlx::SqlitePool,
    ) -> Option<Self> {
        sqlx::query_as!(Self, "SELECT binaries.* FROM binaries INNER JOIN users ON users.id=binaries.user_id WHERE users.username=? AND hash=?", username, hash)
            .fetch_optional(pool)
            .await
            .expect("optional binary fetch by username and hash failed")
    }

    pub async fn get_stats_summary(&self, pool: &sqlx::SqlitePool) -> BinaryStats {
        let (wins_record, losses_record, draws_record, average_turn_run_time_ms_record) = try_join!(
            sqlx::query!("SELECT COUNT(*) AS result FROM players WHERE binary_id=? AND points=2", self.id).fetch_one(pool),
            sqlx::query!("SELECT COUNT(*) AS result FROM players WHERE binary_id=? AND points=0", self.id).fetch_one(pool),
            sqlx::query!("SELECT COUNT(*) AS result FROM players WHERE binary_id=? AND points=1", self.id).fetch_one(pool),
            sqlx::query!(r#"SELECT AVG(run_time_ms) AS "result!: f64" FROM turns JOIN players ON turns.player_id=players.id WHERE players.binary_id=?"#, self.id).fetch_one(pool)
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
