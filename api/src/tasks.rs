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
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug)]
struct CompletedTurn {
    pub action: String,
    pub human_action: String,
    pub state: String,
    pub streams: TurnStreams,
    pub run_time_ms: u128,
    pub started_at: u128,
    pub player_index: usize,
}

#[derive(Clone, Debug)]
struct GamePlayer<T> {
    pub binary_path: PathBuf,
    pub meta: T,
}

#[derive(Clone, Debug)]
struct CompletedGame<T> {
    pub players: Vec<(GamePlayer<T>, u32)>,
    pub turns: Vec<CompletedTurn>,
    pub started_at: u128,
    pub ended_at: u128,
}

pub async fn celery_app_factory() -> Result<std::sync::Arc<celery::Celery<RedisBroker>>> {
    let app = celery::app!(
        broker = RedisBroker { std::env::var("REDIS_URL").expect("REDIS_URL environment variable should be set (via .env or otherwise).") },
        tasks = [play],
        task_routes = [],
        task_retry_for_unexpected = false
    )
    .await?;

    Ok(app)
}

#[celery::task(hard_time_limit = 60, max_retries = 3)]
pub async fn play(game_name: String, players: Vec<(String, String)>) -> TaskResult<()> {
    let game_details = create_game_by_name(&game_name);

    if game_details.is_none() {
        return TaskResult::Err(TaskError::UnexpectedError(format!(
            "Task called with invalid game name, `{}`.",
            game_name
        )));
    }

    let (mut game, player_count, tournament_id) = game_details.unwrap();

    if player_count != players.len() {
        return TaskResult::Err(TaskError::UnexpectedError(format!(
            "Task called with invalid number of players, `{}`.",
            player_count
        )));
    }

    let database_url = std::env::var("DATABASE_URL").with_unexpected_err(|| {
        "DATABASE_URL environment variable should be set (via .env or otherwise)."
    })?;
    let pool = sqlx::SqlitePool::connect(&database_url)
        .await
        .with_expected_err(|| "Failed to connect to database.")?;

    let mut binaries_dir =
        std::env::current_dir().with_unexpected_err(|| "No current directory.")?;
    binaries_dir = binaries_dir.join("binaries");

    // Binary, Path to executable, Username, Container ID
    let mut binaries: Vec<GamePlayer<(Binary, String)>> = vec![];

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

                if !binary_path.is_file() {
                    return TaskResult::Err(TaskError::UnexpectedError(format!(
                        "Binary does not exist: {}-{}.",
                        username, binary.created_at
                    )))
                }

                binaries.push(GamePlayer {
                    binary_path,
                    meta: (binary, username),
                });
            }
        }
    }

    let completed_game = play_game(game, binaries)
        .await
        .with_unexpected_err(|| "Game playing failed.")?;

    let game_instance = GameModel::create(
        tournament_id,
        completed_game.started_at as i64,
        completed_game.started_at as i64,
        &pool,
    )
    .await
    .with_unexpected_err(|| "Failed to insert game.")?;
    let mut players = Vec::with_capacity(completed_game.players.len());

    for (player, score) in &completed_game.players {
        let ranking = sqlx::query_as!(
            Ranking,
            r#"SELECT id, user_id, tournament_id, rating_mu AS "rating_mu: f64", rating_sigma AS "rating_sigma: f64" FROM rankings
            WHERE user_id=? AND tournament_id=?"#,
            player.meta.0.user_id,
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
                    player.meta.0.user_id,
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

        players.push((player, *score, rating));
    }

    let ratings = crate::rating::get_rating_change(players)
        .with_unexpected_err(|| "BBT rating update failed.")?;
    // Player creation.
    for (player, score, rating, new_rating) in ratings {
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
            player.meta.0.user_id,
            player.meta.0.id,
            rating_mu,
            rating_sigma,
            score,
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
            player.meta.0.user_id,
            tournament_id
        )
        .execute(&pool)
        .await
        .with_unexpected_err(|| "Failed to update ranking.")?;
    }

    // Turn Creation
    for (i, turn) in completed_game.turns.iter().enumerate() {
        let turn_number = i as u32 + 1;
        let created_at = turn.started_at as i64;
        let run_time_ms = turn.run_time_ms as i64;
        sqlx::query!(
            "INSERT INTO turns (game_id, turn_number, player_id, created_at, run_time_ms, action, human_action, state, stdout, stderr, stdin)
            VALUES (
                ?, ?,
                (SELECT id FROM players WHERE game_id=? AND user_id=?),
                ?, ?, ?, ?, ?, ?, ?, ?
            )",
            game_instance.id,
            turn_number,
            game_instance.id,
            completed_game.players[turn.player_index].0.meta.0.user_id,
            created_at,
            run_time_ms,
            turn.action,
            turn.human_action,
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

async fn play_game<T>(
    mut game: impl Game,
    players: Vec<GamePlayer<T>>,
) -> Result<CompletedGame<T>> {
    let docker = connect_docker()?;

    let mut container_ids = Vec::with_capacity(players.len());

    for player in &players {
        let container_id = create_container(&docker, &player.binary_path).await?;
        container_ids.push(container_id);
    }

    let mut turns = vec![];

    let started_at = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis();

    let scores = loop {
        let input = game.get_game_state();

        match input {
            GameState::Complete(s) => {
                break s;
            }
            GameState::Turn(turn_data) => {
                let container_id = &container_ids[turn_data.player_index];

                let i = SystemTime::now();
                let execution_result =
                    exec_container_binary(&docker, container_id, &turn_data.stdin).await?;
                let duration = SystemTime::now().duration_since(i)?;

                let stdout = execution_result.stdout;
                let mut action = "";
                for line in stdout.split("\n") {
                    let line = line.trim();
                    if line != "" {
                        action = line;
                    }
                }

                let turn_result = game.respond(action);

                turns.push(CompletedTurn {
                    human_action: turn_result.1,
                    action: action.to_owned(),
                    started_at: i.duration_since(UNIX_EPOCH)?.as_millis(),
                    state: if execution_result.timed_out {
                        "timed_out".to_owned()
                    } else {
                        match turn_result.0 {
                            TurnResult::Illegal => "illegal".to_owned(),
                            TurnResult::Invalid => "invalid".to_owned(),
                            TurnResult::Legal => "legal".to_owned(),
                        }
                    },
                    run_time_ms: duration.as_millis(),
                    streams: TurnStreams {
                        stdin: turn_data.stdin,
                        stdout: stdout.to_owned(),
                        stderr: execution_result.stderr.to_owned(),
                    },
                    player_index: turn_data.player_index,
                });
            }
        };
    };

    let ended_at = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis();

    for container_id in container_ids {
        teardown_container(&docker, &container_id).await?;
    }

    Ok(CompletedGame {
        started_at,
        ended_at,
        players: players
            .into_iter()
            .zip(scores.into_iter())
            .collect::<Vec<_>>(),
        turns,
    })
}
