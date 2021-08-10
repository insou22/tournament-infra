use crate::models::{
    binary::{Binary, BinaryResponse},
    Ranking,
};
use rocket::response::status::Unauthorized;
use rocket::tokio::try_join;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct TournamentStats {
    pub ranking: i32,
    pub wins: i32,
    pub losses: i32,
    pub draws: i32,
    pub win_loss: f64,
    pub rating: i64,
    pub average_turn_run_time_ms: f64,
}

#[derive(Serialize)]
pub struct UserProfile {
    #[serde(flatten)]
    pub user: User,
    pub current_tournament_stats_summary: Option<TournamentStats>,
    pub current_binary: Option<BinaryResponse>,
}

#[derive(Serialize)]
pub struct UserInfo {
    pub username: String,
    pub display_name: String,
    pub current_rating: Option<i64>,
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

    pub async fn get_userinfo(&self, tournament_id: i64, pool: &sqlx::SqlitePool) -> UserInfo {
        let ranking = sqlx::query_as!(
            Ranking,
            "SELECT * FROM rankings WHERE user_id=? AND tournament_id=?",
            self.id,
            tournament_id
        )
        .fetch_optional(pool)
        .await
        .expect("optional ranking fetch failed");

        UserInfo {
            username: self.username.to_owned(),
            display_name: self.display_name.to_owned(),
            current_rating: ranking.and_then(|r| Some(r.rating)),
        }
    }

    pub async fn get_profile(&self, tournament_id: i64, pool: &sqlx::SqlitePool) -> UserProfile {
        // Must be successfully compiled to be the user's current binary (even if the user is viewing their own profile, as this binary is used in games).
        let binary = sqlx::query_as!(
            Binary,
            "SELECT * FROM binaries WHERE user_id=? AND tournament_id=? AND compile_result='success' ORDER BY created_at DESC",
            self.id,
            tournament_id
        )
        .fetch_optional(pool)
        .await
        .expect("optional binary fetch failed");

        UserProfile {
            current_binary: match binary {
                Some(binary) => Some(BinaryResponse {
                    stats_summary: binary.get_stats_summary(pool).await,
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
                    sqlx::query!("SELECT COUNT(*) + 1 AS result FROM rankings WHERE rating > ? AND tournament_id=?", ranking.rating, tournament_id).fetch_one(pool),
                    sqlx::query!("SELECT COUNT(*) AS result FROM players JOIN games ON games.id=players.game_id WHERE user_id=? AND tournament_id=? AND points=2", self.id, tournament_id).fetch_one(pool),
                    sqlx::query!("SELECT COUNT(*) AS result FROM players JOIN games ON games.id=players.game_id WHERE user_id=? AND tournament_id=? AND points=0", self.id, tournament_id).fetch_one(pool),
                    sqlx::query!("SELECT COUNT(*) AS result FROM players JOIN games ON games.id=players.game_id WHERE user_id=? AND tournament_id=? AND points=1", self.id, tournament_id).fetch_one(pool),
                    sqlx::query!(r#"SELECT AVG(run_time_ms) AS "result!: f64" FROM turns JOIN games ON turns.game_id=games.id JOIN players ON turns.player_id=players.id WHERE players.user_id=? AND games.tournament_id=?"#, self.id, tournament_id).fetch_one(pool)
                ).expect("a tournament stats fetch failed");

                Some(TournamentStats {
                    ranking: position_record.result,
                    rating: ranking.rating,
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

#[rocket::async_trait]
impl<'r> rocket::request::FromRequest<'r> for User {
    type Error = Unauthorized<()>;

    async fn from_request(
        request: &'r rocket::Request<'_>,
    ) -> rocket::request::Outcome<Self, Self::Error> {
        match request.cookies().get_private("zid") {
            None => rocket::request::Outcome::Failure((
                rocket::http::Status::Unauthorized,
                Unauthorized(None),
            )),
            Some(zid_cookie) => {
                let zid = zid_cookie.value();
                let pool = request
                    .guard::<&rocket::State<sqlx::SqlitePool>>()
                    .await
                    .unwrap();
                let user = User::get_by_username(zid, pool.inner()).await;
                match user {
                    None => rocket::request::Outcome::Failure((
                        rocket::http::Status::Unauthorized,
                        Unauthorized(None),
                    )),
                    Some(u) => rocket::request::Outcome::Success(u),
                }
            }
        }
    }
}
