use crate::paginate::Paginatable;
use serde::Serialize;

#[derive(Serialize)]
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
    pub rating_before_game: i64,
    pub rating_change: Option<i64>,
    pub result: Option<String>,
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
    pub completed_at: Option<i64>,
}

impl Game {
    pub async fn get_by_id(id: i64, pool: &sqlx::SqlitePool) -> Option<Self> {
        sqlx::query_as!(Self, "SELECT * FROM games WHERE id=?", id)
            .fetch_optional(pool)
            .await
            .expect("game fetch failed")
    }

    pub async fn get_players(&self, pool: &sqlx::SqlitePool) -> Vec<Player> {
        sqlx::query_as!(
            Player,
            r#"SELECT
                binaries.hash AS binary_hash,
                users.username AS username,
                users.display_name AS display_name,
                players.rating_before_game AS rating_before_game,
                players.rating_change AS rating_change,
                CASE players.points
                    WHEN 2 THEN 'won'
                    WHEN 1 THEN 'drew'
                    WHEN 0 THEN 'lost'
                END AS "result?: String"
            FROM players
            JOIN users ON players.user_id=users.id
            JOIN binaries ON players.binary_id=binaries.id
            WHERE players.game_id=?"#,
            self.id
        )
        .fetch_all(pool)
        .await
        .expect("players fetch all failed")
    }

    pub async fn get_turns(&self, pool: &sqlx::SqlitePool) -> Vec<Turn> {
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
        .fetch_all(pool)
        .await
        .expect("turns fetch all failed")
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

        return turns;
    }
}
