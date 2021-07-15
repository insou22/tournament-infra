use crate::models::{
    binary::{Binary, BinaryWithStats},
    Ranking,
};
use rocket::tokio::try_join;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct TournamentStats {
    pub ranking: i32,
    pub wins: i32,
    pub losses: i32,
    pub draws: i32,
    pub win_loss: f64,
    pub elo: i64,
    pub average_turn_run_time_ms: f64,
}

#[derive(Serialize)]
pub struct UserProfile {
    #[serde(flatten)]
    pub user: User,
    pub current_tournament_stats_summary: Option<TournamentStats>,
    pub current_binary: Option<BinaryWithStats>,
}

#[derive(Serialize)]
pub struct UserInfo {
    pub username: String,
    pub display_name: String,
    pub current_elo: Option<i64>,
}

#[derive(Serialize, Clone)]
pub struct User {
    #[serde(skip)]
    pub id: i64,
    pub username: String,
    pub display_name: String,
}

impl User {
    pub async fn get_by_username(username: &str, pool: &sqlx::SqlitePool) -> Option<Self> {
        sqlx::query_as!(Self, "SELECT * FROM users WHERE username=?", username)
            .fetch_optional(pool)
            .await
            .expect("optional user fetch failed")
    }

    pub async fn get_userinfo(&self, pool: &sqlx::SqlitePool) -> UserInfo {
        let ranking = sqlx::query_as!(
            Ranking,
            "SELECT * FROM rankings WHERE user_id=? AND tournament_id=?",
            self.id,
            1
        )
        .fetch_optional(pool)
        .await
        .expect("optional ranking fetch failed");

        UserInfo {
            username: self.username.to_owned(),
            display_name: self.display_name.to_owned(),
            current_elo: ranking.and_then(|r| Some(r.elo)),
        }
    }

    pub async fn get_profile(&self, tournament_id: i64, pool: &sqlx::SqlitePool) -> UserProfile {
        let binary = sqlx::query_as!(
            Binary,
            "SELECT * FROM binaries WHERE user_id=? AND tournament_id=? ORDER BY created_at DESC LIMIT 1",
            self.id,
            tournament_id
        )
        .fetch_optional(pool)
        .await
        .expect("optional binary fetch failed");

        UserProfile {
            current_binary: match binary {
                Some(binary) => Some(BinaryWithStats {
                    stats_summary: binary.get_stats_summary(tournament_id, pool).await,
                    binary,
                }),
                None => None,
            },
            current_tournament_stats_summary: self
                .get_tournament_stats_summary(tournament_id, pool)
                .await,
            user: self.clone(),
        }
    }

    async fn get_tournament_stats_summary(
        &self,
        tournament_id: i64,
        pool: &sqlx::SqlitePool,
    ) -> Option<TournamentStats> {
        match sqlx::query_as!(
            Ranking,
            "SELECT * FROM rankings WHERE user_id=? AND tournament_id=?",
            self.id,
            tournament_id
        )
        .fetch_optional(pool)
        .await
        .expect("ranking fetch failed")
        {
            None => None,
            Some(ranking) => {
                let (position_record, wins_record, losses_record, draws_record, average_turn_run_time_ms_record) = try_join!(
                    sqlx::query!("SELECT COUNT(*) + 1 AS result FROM rankings WHERE elo > ? AND tournament_id=?", ranking.elo, tournament_id).fetch_one(pool),
                    sqlx::query!("SELECT COUNT(*) AS result FROM games WHERE user_id=? AND tournament_id=? AND points=?", self.id, tournament_id, 2).fetch_one(pool),
                    sqlx::query!("SELECT COUNT(*) AS result FROM games WHERE user_id=? AND tournament_id=? AND points=?", self.id, tournament_id, 1).fetch_one(pool),
                    sqlx::query!("SELECT COUNT(*) AS result FROM games WHERE user_id=? AND tournament_id=? AND points=?", self.id, tournament_id, 0).fetch_one(pool),
                    sqlx::query!(r#"SELECT AVG(time_taken_ms) AS "result!: f64" FROM turns JOIN games ON turns.game_id=games.id WHERE turns.user_id=? AND games.tournament_id=?"#, self.id, tournament_id).fetch_one(pool)
                ).expect("a tournament stats fetch failed");
                
                Some(TournamentStats {
                    ranking: position_record.result,
                    elo: ranking.elo,
                    win_loss: wins_record.result as f64 / losses_record.result as f64,
                    wins: wins_record.result,
                    losses: losses_record.result,
                    draws: draws_record.result,
                    average_turn_run_time_ms: average_turn_run_time_ms_record.result,
                })
            }
        }
    }
}
