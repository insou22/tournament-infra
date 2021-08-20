use crate::errors::*;
use crate::paginate::Paginatable;
use rocket::tokio::try_join;
use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
pub struct BinaryStats {
    pub wins: i32,
    pub losses: i32,
    pub draws: i32,
    pub win_loss: f64,
    pub win_loss_ratio_percentage_change: Option<f64>,
    pub average_turn_run_time_ms: f64,
    pub average_turn_run_time_ms_percentage_change: Option<f64>,
}

#[derive(Serialize, Clone, Debug)]
pub struct BinaryResponse {
    #[serde(flatten)]
    pub binary: Binary,
    pub stats_summary: BinaryStats,
}

impl Paginatable for BinaryResponse {
    type CursorType = i64;
    fn get_cursor(&self) -> Self::CursorType {
        self.binary.created_at
    }
}

#[derive(Serialize, Clone, Debug)]
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
        conn: &mut sqlx::SqliteConnection,
    ) -> Result<Option<Self>> {
        Ok(sqlx::query_as!(Self, "SELECT binaries.* FROM binaries INNER JOIN users ON users.id=binaries.user_id WHERE users.username=? AND hash=?", username, hash)
            .fetch_optional(conn)
            .await?)
    }

    async fn get_predecessor(&self, conn: &mut sqlx::SqliteConnection) -> Result<Option<Self>> {
        Ok(sqlx::query_as!(
            Self,
            "SELECT * FROM binaries WHERE created_at<? AND user_id=? AND tournament_id=? AND compile_result='success' ORDER BY created_at DESC",
            self.created_at,
            self.user_id,
            self.tournament_id
        )
        .fetch_optional(conn)
        .await?)
    }

    async fn get_stats_summary_without_changes(
        &self,
        conn: &mut sqlx::SqliteConnection,
    ) -> Result<BinaryStats> {
        let (wins_record, losses_record, draws_record, average_turn_run_time_ms_record) = try_join!(
            sqlx::query!("SELECT COUNT(*) AS result FROM players WHERE binary_id=? AND points=2", self.id).fetch_one(conn),
            sqlx::query!("SELECT COUNT(*) AS result FROM players WHERE binary_id=? AND points=0", self.id).fetch_one(conn),
            sqlx::query!("SELECT COUNT(*) AS result FROM players WHERE binary_id=? AND points=1", self.id).fetch_one(conn),
            sqlx::query!(r#"SELECT AVG(run_time_ms) AS "result!: f64" FROM turns JOIN players ON turns.player_id=players.id WHERE players.binary_id=?"#, self.id).fetch_one(conn)
        )?;

        Ok(BinaryStats {
            wins: wins_record.result,
            losses: losses_record.result,
            draws: draws_record.result,
            win_loss: wins_record.result as f64 / losses_record.result as f64,
            average_turn_run_time_ms: average_turn_run_time_ms_record.result,
            average_turn_run_time_ms_percentage_change: None,
            win_loss_ratio_percentage_change: None,
        })
    }

    pub async fn get_stats_summary(
        &self,
        conn: &mut sqlx::SqliteConnection,
    ) -> Result<BinaryStats> {
        // When doing this for a large amount of binaries, this is ridiculously inefficient.
        // But the only view of multiple binaries that exists is a single user's binaries.
        // If one person hits an amount that this becomes unreasonable, I will a) be impressed, and b) optimise this :P
        let mut stats = self.get_stats_summary_without_changes(conn).await?;

        let predecessor = self.get_predecessor(conn).await?;
        if let Some(predecessor) = predecessor {
            let predecessor_stats = predecessor.get_stats_summary_without_changes(conn).await?;

            stats.win_loss_ratio_percentage_change =
                Some((stats.win_loss / predecessor_stats.win_loss * 100f64) - 100f64);
            stats.average_turn_run_time_ms_percentage_change = Some(
                (stats.average_turn_run_time_ms / predecessor_stats.average_turn_run_time_ms
                    * 100f64)
                    - 100f64,
            );
        }
        Ok(stats)
    }
}
