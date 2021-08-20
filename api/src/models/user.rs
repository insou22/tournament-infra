use crate::errors::*;
use crate::models::{
    binary::{Binary, BinaryResponse},
    ranking::Ranking,
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
    pub rating_mu: f64,
    pub rating_sigma: f64,
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
    pub current_rating_mu: Option<f64>,
    pub current_rating_sigma: Option<f64>,
}

#[derive(Serialize, Clone)]
pub struct User {
    #[serde(skip)]
    pub id: i64,
    pub username: String,
    pub display_name: String,
}

impl User {
    pub async fn get_by_username(
        username: &str,
        conn: &mut sqlx::SqliteConnection,
    ) -> Result<Option<Self>> {
        Ok(
            sqlx::query_as!(Self, "SELECT * FROM users WHERE username=?", username)
                .fetch_optional(conn)
                .await?,
        )
    }

    pub async fn get_userinfo(
        &self,
        tournament_id: i64,
        conn: &mut sqlx::SqliteConnection,
    ) -> Result<UserInfo> {
        let ranking: Option<Ranking> = self.get_ranking(tournament_id, conn).await?;

        Ok(UserInfo {
            username: self.username.to_owned(),
            display_name: self.display_name.to_owned(),
            current_rating_mu: ranking.as_ref().and_then(|r| Some(r.rating_mu as f64)),
            current_rating_sigma: ranking.as_ref().and_then(|r| Some(r.rating_sigma as f64)),
        })
    }

    pub async fn get_ranking(
        &self,
        tournament_id: i64,
        conn: &mut sqlx::SqliteConnection,
    ) -> Result<Option<Ranking>> {
        Ok(sqlx::query_as!(
            Ranking,
            r#"SELECT id, user_id, tournament_id, rating_mu AS "rating_mu: f64", rating_sigma AS "rating_sigma: f64" FROM rankings
            WHERE user_id=? AND tournament_id=?"#,
            self.id,
            tournament_id
        )
        .fetch_optional(conn)
        .await?)
    }

    pub async fn get_profile(
        &self,
        tournament_id: i64,
        conn: &mut sqlx::SqliteConnection,
    ) -> Result<UserProfile> {
        // Must be successfully compiled to be the user's current binary (even if the user is viewing their own profile, as this binary is used in games).
        let binary = sqlx::query_as!(
            Binary,
            "SELECT * FROM binaries WHERE user_id=? AND tournament_id=? AND compile_result='success' ORDER BY created_at DESC",
            self.id,
            tournament_id
        )
        .fetch_optional(conn)
        .await?;

        Ok(UserProfile {
            current_binary: match binary {
                Some(binary) => Some(BinaryResponse {
                    stats_summary: binary.get_stats_summary(conn).await?,
                    binary,
                }),
                None => None,
            },
            current_tournament_stats_summary: self
                .get_tournament_stats_summary(tournament_id, conn)
                .await?,
            user: self.clone(),
        })
    }

    async fn get_tournament_stats_summary(
        &self,
        tournament_id: i64,
        conn: &mut sqlx::SqliteConnection,
    ) -> Result<Option<TournamentStats>> {
        Ok(match self.get_ranking(tournament_id, conn).await? {
            None => None,
            Some(ranking) => {
                let stats_record = sqlx::query!(
                    r#"SELECT
                        (SELECT COUNT(*) + 1 FROM rankings WHERE rating_mu > ? AND tournament_id=?) AS ranking,
                        (SELECT COUNT(*) FROM players JOIN games ON games.id=players.game_id WHERE user_id=? AND tournament_id=? AND points=2) AS wins,
                        (SELECT COUNT(*) FROM players JOIN games ON games.id=players.game_id WHERE user_id=? AND tournament_id=? AND points=0) AS losses,
                        (SELECT COUNT(*) FROM players JOIN games ON games.id=players.game_id WHERE user_id=? AND tournament_id=? AND points=1) AS draws,
                        (SELECT AVG(run_time_ms) AS "result!: f64" FROM turns JOIN games ON turns.game_id=games.id JOIN players ON turns.player_id=players.id WHERE players.user_id=? AND games.tournament_id=?) AS "average_turn_run_time_ms!: f64""#,
                    ranking.rating_mu, tournament_id,
                    self.id, tournament_id,
                    self.id, tournament_id,
                    self.id, tournament_id,
                    self.id, tournament_id
                ).fetch_one(conn).await?;

                Some(TournamentStats {
                    ranking: stats_record.ranking,
                    rating_mu: ranking.rating_mu,
                    rating_sigma: ranking.rating_sigma,
                    win_loss: stats_record.wins as f64 / stats_record.losses as f64,
                    wins: stats_record.wins,
                    losses: stats_record.losses,
                    draws: stats_record.draws,
                    average_turn_run_time_ms: stats_record.average_turn_run_time_ms,
                })
            }
        })
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
                let conn = pool.inner().acquire().await.unwrap(); // TODO: Don't unwrap this...
                let user = User::get_by_username(zid, &mut conn).await.unwrap(); // TODO: Don't unwrap this...
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
