use rocket::tokio::try_join;
use serde::Serialize;

#[derive(Serialize)]
pub struct BinaryStats {
    pub wins: i64,
    pub losses: i64,
    pub draws: i64,
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
        let stats_record = sqlx::query!(
            r#"SELECT
                SUM(CASE players.points WHEN 2 THEN 1 ELSE 0 END) AS "wins!: i64",
                SUM(CASE players.points WHEN 0 THEN 1 ELSE 0 END) AS "losses!: i64",
                SUM(CASE players.points WHEN 1 THEN 1 ELSE 0 END) AS "draws!: i64",
                AVG(turns.time_taken_ms) AS "average_turn_run_time_ms!: f64"
            FROM players
            JOIN turns ON turns.player_id=players.id
            WHERE players.binary_id=?"#,
            self.id
        )
        .fetch_one(pool)
        .await
        .expect("binary stats fetch failed");

        return BinaryStats {
            wins: stats_record.wins,
            losses: stats_record.losses,
            draws: stats_record.draws,
            win_loss: stats_record.wins as f64 / stats_record.losses as f64,
            average_turn_run_time_ms: stats_record.average_turn_run_time_ms,
            average_turn_run_time_ms_percentage_change: None, // TODO: Get change from previous binary in query.
            win_loss_ratio_percentage_change: None,
        };
    }
}
