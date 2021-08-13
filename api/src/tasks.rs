use crate::game::{Game, GameState, TurnResult};
use crate::games::create_game_by_name;
use crate::isolator::{
    connect_docker, create_container, exec_container_binary, teardown_container,
};
use crate::models::binary::Binary;
use crate::models::game::TurnStreams;
use celery::prelude::*;
use sqlx::Connection;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

struct GameBinary {
    pub binary: Binary,
    pub username: String,
    pub container_id: String,
}

struct TurnInsert<'a> {
    pub action: String,
    pub state: String,
    pub streams: TurnStreams,
    pub run_time_ms: u32,
    pub created_at: i64,
    pub player: &'a GameBinary,
}

#[celery::task(hard_time_limit = 60, max_retries = 3)]
pub async fn play(game_name: String, players: Vec<(String, String)>) -> TaskResult<()> {
    let game_details = create_game_by_name(&game_name);

    if game_details.is_none() {
        return TaskResult::Err(TaskError::ExpectedError(format!(
            "Task called with invalid game name, `{}`.",
            game_name
        )));
    }

    let (mut game, player_count, tournament_id) = game_details.unwrap();

    if player_count != players.len() {
        return TaskResult::Err(TaskError::ExpectedError(format!(
            "Task called with invalid number of players, `{}`.",
            player_count
        )));
    }

    let database_url = std::env::var("DATABASE_URL").with_expected_err(|| {
        "DATABASE_URL environment variable should be set (via .env or otherwise)."
    })?;
    let pool = sqlx::SqlitePool::connect(&database_url)
        .await
        .with_expected_err(|| "Failed to connect to database.")?;
    let docker = connect_docker().with_unexpected_err(|| "Failed to connect to docker.")?;

    let mut binaries_dir =
        std::env::current_dir().with_unexpected_err(|| "No current directory.")?;
    binaries_dir = binaries_dir.join("binaries");

    // Binary, Path to executable, Username, Container ID
    let mut binaries: Vec<GameBinary> = vec![];

    for (username, binary_hash) in players {
        let binary = Binary::get_by_username_and_hash(&username, &binary_hash, &pool).await;

        match binary {
            None => {
                return TaskResult::Err(TaskError::UnexpectedError(format!(
                    "Task called with non-existent binary hash ({})/username ({}).",
                    binary_hash, username
                )))
            }
            Some(binary) => {
                let binary_path =
                    binaries_dir.join(Path::new(&format!("{}-{}", username, binary.created_at)));

                let container_id = create_container(&docker, &binary_path)
                    .await
                    .with_unexpected_err(|| "Failed to create container.")?;

                binaries.push(GameBinary {
                    binary,
                    username,
                    container_id,
                });
            }
        }
    }

    let mut scores = None;
    let mut turns: Vec<TurnInsert> = vec![];

    let game_start_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .with_unexpected_err(|| "Timing failure.")?
        .as_millis() as i64;

    while scores.is_none() {
        let input = game.get_game_state();

        match input {
            GameState::Complete(s) => {
                scores = Some(s);
            }
            GameState::Turn(turn_data) => {
                let player_container = &binaries[turn_data.player_index];

                let i = SystemTime::now();
                let execution_result = exec_container_binary(
                    &docker,
                    &player_container.container_id,
                    &turn_data.stdin,
                )
                .await
                .with_unexpected_err(|| "Failed to execute binary in container.")?;
                let duration = SystemTime::now()
                    .duration_since(i)
                    .with_unexpected_err(|| "Timing failure.")?;

                let stdout = execution_result.stdout;
                let mut card = "";
                for line in stdout.split("\n") {
                    let line = line.trim();
                    if line != "" {
                        card = line;
                    }
                }

                let turn_result = game.respond(card);

                turns.push(TurnInsert {
                    action: turn_result.1,
                    created_at: i
                        .duration_since(UNIX_EPOCH)
                        .with_unexpected_err(|| "Timing failure.")?
                        .as_millis() as i64,
                    state: if execution_result.timed_out {
                        "timed_out".to_owned()
                    } else {
                        match turn_result.0 {
                            TurnResult::Illegal => "illegal".to_owned(),
                            TurnResult::Invalid => "invalid".to_owned(),
                            TurnResult::Legal => "legal".to_owned(),
                        }
                    },
                    run_time_ms: duration.as_millis() as u32,
                    streams: TurnStreams {
                        stdin: turn_data.stdin,
                        stdout: stdout.to_owned(),
                        stderr: execution_result.stderr.to_owned(),
                    },
                    player: &player_container,
                });
            }
        };
    }

    let game_end_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .with_unexpected_err(|| "Timing failure.")?
        .as_millis() as i64;

    for player_container in &binaries {
        teardown_container(&docker, &player_container.container_id)
            .await
            .with_unexpected_err(|| "Failed to teardown container.")?;
    }

    sqlx::query!(
        "INSERT INTO games (tournament_id, created_at, completed_at)
        VALUES (?, ?, ?)",
        tournament_id,
        game_start_time,
        game_end_time
    )
    .execute(&pool)
    .await
    .with_unexpected_err(|| "Failed to insert game.")?;

    let scores = scores.unwrap();
    for (player_container, score) in binaries.iter().zip(scores.iter()) {
        sqlx::query!(
            "INSERT INTO players (game_id, user_id, binary_id, rating_before_game, points, rating_change)
            VALUES (
                (SELECT id FROM games WHERE created_at=?),
                ?, ?, ?, ?, ?
            )",
            game_start_time,
            player_container.binary.user_id,
            player_container.binary.id,
            1000, // TODO: Find way to grab ratings without race conditions...
            *score,
            13
        )
        .execute(&pool)
        .await
        .with_unexpected_err(|| "Failed to insert turn.")?;
    }

    for (i, turn) in turns.iter().enumerate() {
        let turn_number = i as u32 + 1;
        sqlx::query!(
            "INSERT INTO turns (game_id, turn_number, player_id, created_at, run_time_ms, action, state, stdout, stderr, stdin)
            VALUES (
                (SELECT id FROM games WHERE created_at=?),
                ?,
                (SELECT id FROM players WHERE game_id=(SELECT id FROM games WHERE created_at=?) AND user_id=?),
                ?, ?, ?, ?, ?, ?, ?
            )",
            game_start_time,
            turn_number,
            game_start_time,
            turn.player.binary.user_id,
            turn.created_at,
            turn.run_time_ms,
            turn.action,
            turn.state,
            turn.streams.stdout,
            turn.streams.stderr,
            turn.streams.stdin
        )
        .execute(&pool)
        .await
        .with_unexpected_err(|| "Failed to insert turn.")?;
    }

    TaskResult::Ok(())
}
