use serde::Serialize;

#[derive(Serialize)]
pub struct TurnStreams {
    pub stdin: String,
    pub stdout: String,
    pub stderr: String,
}

#[derive(Serialize)]
pub struct Turn {
    username: String,
    #[serde(rename = "move")]
    move_string: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    streams: Option<TurnStreams>,
    run_time: i64,
}

#[derive(Serialize)]
pub struct Player {
    pub binary_hash: String,
    pub username: String,
    pub display_name: String,
    pub rating_before_game: i64,
    pub rating_change: Option<i64>,
    pub result: Option<String>
}

#[derive(Serialize)]
pub struct GameResponse {
    #[serde(flatten)]
    pub game: Game,
    pub players: Vec<Player>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub turns: Option<Vec<Turn>>,
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
        .expect("player fetch all failed")
    }
}
