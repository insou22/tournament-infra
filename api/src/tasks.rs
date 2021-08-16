use crate::errors::*;
use crate::game::{Game, GameState, TurnResult};
use crate::games::create_game_by_name;
use crate::isolator::{
    connect_docker, create_container, exec_container_binary, teardown_container,
};
use crate::models::binary::Binary;
use crate::models::game::{Game as GameModel, TurnStreams};
use crate::models::ranking::Ranking;
use celery::prelude::*;
use sqlx::Connection;
use std::collections::HashMap;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

const RATER_BETA: f64 = 25.0 / 6.0;

#[derive(Clone, Debug)]
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

    // Game creation.
    let game_instance = GameModel::create(tournament_id, game_start_time, game_end_time, &pool)
        .await
        .with_unexpected_err(|| "Failed to insert game.")?;

    let scores = scores.unwrap();

    let mut ratings = Vec::with_capacity(binaries.len());

    for (player, score) in binaries.iter().zip(scores.iter()) {
        let ranking = sqlx::query_as!(
            Ranking,
            r#"SELECT id, user_id, tournament_id, rating_mu AS "rating_mu: f64", rating_sigma AS "rating_sigma: f64" FROM rankings
            WHERE user_id=? AND tournament_id=?"#,
            player.binary.user_id,
            tournament_id
        )
        .fetch_optional(&pool)
        .await
        .with_unexpected_err(|| "Failed to fetch ranking.")?;

        let rating = match ranking {
            Some(r) => bbt::Rating::new(r.rating_mu, r.rating_sigma),
            None => {
                let rating = bbt::Rating::default();

                let mu = rating.mu();
                let sigma = rating.sigma();

                sqlx::query!(
                    "INSERT INTO rankings (user_id, tournament_id, rating_mu, rating_sigma)
                    VALUES (?, ?, ?, ?)",
                    player.binary.user_id,
                    tournament_id,
                    mu,
                    sigma
                )
                .execute(&pool)
                .await
                .with_unexpected_err(|| "Failed to insert new ranking.");

                rating
            }
        };

        ratings.push((score, rating));
    }
    let rater = bbt::Rater::new(RATER_BETA);
    let new_ratings = rater
        .update_ratings(
            ratings
                .iter()
                .map(|(s, r)| vec![r.clone()])
                .collect::<Vec<_>>(),
            ratings
                .iter()
                .map(|(&s, r)| 2 - s as usize)
                .collect::<Vec<_>>(),
        )
        .or_else::<Error, _>(|e| Err(e.into()))
        .with_unexpected_err(|| "BBT rating update failed.")?;

    // Player creation.
    for ((player_container, (score, rating)), new_rating_vec) in
        binaries.iter().zip(ratings.iter()).zip(new_ratings.iter())
    {
        let new_rating = &new_rating_vec[0];
        // SQLx hates temporary values, so have to do this all here...
        let new_rating_mu = new_rating.mu();
        let new_rating_sigma = new_rating.sigma();
        let rating_mu = rating.mu();
        let rating_sigma = rating.sigma();
        let rating_change_mu = rating_mu - new_rating_mu;
        let rating_change_sigma = rating_sigma - new_rating_sigma;

        sqlx::query!(
            "INSERT INTO players (game_id, user_id, binary_id, rating_mu_before_game, rating_sigma_before_game, points, rating_mu_change, rating_sigma_change)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
            game_instance.id,
            player_container.binary.user_id,
            player_container.binary.id,
            rating_mu,
            rating_sigma,
            *score,
            rating_change_mu,
            rating_change_sigma
        )
        .execute(&pool)
        .await
        .with_unexpected_err(|| "Failed to insert player.")?;

        sqlx::query!(
            "UPDATE rankings SET rating_mu=?, rating_sigma=? WHERE user_id=? AND tournament_id=?",
            new_rating_mu,
            new_rating_sigma,
            player_container.binary.user_id,
            tournament_id
        )
        .execute(&pool)
        .await
        .with_unexpected_err(|| "Failed to update ranking.")?;
    }

    // Turn Creation
    for (i, turn) in turns.iter().enumerate() {
        let turn_number = i as u32 + 1;
        sqlx::query!(
            "INSERT INTO turns (game_id, turn_number, player_id, created_at, run_time_ms, action, state, stdout, stderr, stdin)
            VALUES (
                ?, ?,
                (SELECT id FROM players WHERE game_id=? AND user_id=?),
                ?, ?, ?, ?, ?, ?, ?
            )",
            game_instance.id,
            turn_number,
            game_instance.id,
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
