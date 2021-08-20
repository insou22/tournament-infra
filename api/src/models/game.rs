use crate::errors::*;
use crate::paginate::Paginatable;
use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
pub struct TurnStreams {
    pub stdin: String,
    pub stdout: String,
    pub stderr: String,
}

#[derive(Serialize)]
pub struct Turn {
    #[serde(skip)]
    pub user_id: i64,
    pub username: String,
    pub action: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub streams: Option<TurnStreams>,
    pub run_time_ms: i64,
}

#[derive(Serialize)]
pub struct Player {
    pub binary_hash: String,
    pub username: String,
    pub display_name: String,
    pub rating_mu_before_game: f64,
    pub rating_sigma_before_game: f64,
    pub rating_mu_change: f64,
    pub rating_sigma_change: f64,
    pub result: String,
}

#[derive(Serialize)]
pub struct GameResponse {
    #[serde(flatten)]
    pub game: Game,
    pub players: Vec<Player>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub turns: Option<Vec<Turn>>,
}

impl Paginatable for GameResponse {
    type CursorType = i64;
    fn get_cursor(&self) -> Self::CursorType {
        self.game.created_at
    }
}

#[derive(Serialize)]
pub struct Game {
    pub id: i64,
    #[serde(skip)]
    pub tournament_id: i64,
    pub created_at: i64,
    pub completed_at: i64,
}

impl Game {
    pub async fn get_by_id(id: i64, conn: &mut sqlx::SqliteConnection) -> Result<Option<Self>> {
        Ok(sqlx::query_as!(Self, "SELECT * FROM games WHERE id=?", id)
            .fetch_optional(conn)
            .await?)
    }

    pub async fn create(
        tournament_id: i64,
        created_at: i64,
        completed_at: i64,
        conn: &mut sqlx::SqliteConnection,
    ) -> Result<Self> {
        Ok(sqlx::query_as!(
            Self,
            "INSERT INTO games (tournament_id, created_at, completed_at)
            VALUES (?, ?, ?);
            SELECT * FROM games WHERE created_at=?",
            tournament_id,
            created_at,
            completed_at,
            created_at
        )
        .fetch_one(conn)
        .await?)
    }

    pub async fn get_players(&self, conn: &mut sqlx::SqliteConnection) -> Result<Vec<Player>> {
        Ok(sqlx::query_as!(
            Player,
            r#"SELECT
                binaries.hash AS binary_hash,
                users.username AS username,
                users.display_name AS display_name,
                players.rating_mu_before_game AS "rating_mu_before_game: f64",
                players.rating_sigma_before_game AS "rating_sigma_before_game: f64",
                players.rating_mu_change AS "rating_mu_change: f64",
                players.rating_sigma_change AS "rating_sigma_change: f64",
                CASE players.points
                    WHEN 2 THEN 'won'
                    WHEN 1 THEN 'drew'
                    WHEN 0 THEN 'lost'
                END AS "result!: String"
            FROM players
            JOIN users ON players.user_id=users.id
            JOIN binaries ON players.binary_id=binaries.id
            WHERE players.game_id=?"#,
            self.id
        )
        .fetch_all(conn)
        .await?)
    }

    pub async fn get_turns(&self, conn: &mut sqlx::SqliteConnection) -> Result<Vec<Turn>> {
        let mut turns = vec![];

        for turn_record in sqlx::query!(
            "SELECT
                users.id AS user_id,
                username,
                action,
                state,
                run_time_ms,
                stdin,
                stdout,
                stderr
            FROM turns
            JOIN players ON turns.player_id=players.id
            JOIN users ON players.user_id=users.id
            WHERE turns.game_id=?",
            self.id
        )
        .fetch_all(conn)
        .await?
        {
            turns.push(Turn {
                user_id: turn_record.user_id,
                username: turn_record.username,
                action: turn_record.action,
                run_time_ms: turn_record.run_time_ms,
                state: Some(turn_record.state),
                streams: Some(TurnStreams {
                    stdin: turn_record.stdin,
                    stdout: turn_record.stdout,
                    stderr: turn_record.stderr,
                }),
            })
        }

        Ok(turns)
    }
}
