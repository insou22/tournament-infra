use crate::game::{Game, GameState, TurnResult};
use crate::games::create_game_by_name;
use crate::isolator::{
    connect_docker, create_container, exec_container_binary, teardown_container,
};
use crate::models::binary::Binary;
use celery::prelude::*;
use sqlx::Connection;
use std::path::Path;
use std::time::Instant;

struct PlayerContainer {
    pub username: String,
    pub binary_hash: String,
    pub container_id: String,
}

#[celery::task(hard_time_limit = 60, max_retries = 3, retry_for_unexpected = true)]
pub async fn play(game_name: String, players: Vec<(String, String)>) -> TaskResult<()> {
    let game_details = create_game_by_name(&game_name);

    if game_details.is_none() {
        return TaskResult::Err(TaskError::ExpectedError(format!(
            "Task called with invalid game name, `{}`.",
            game_name
        )));
    }

    let (mut game, player_count) = game_details.unwrap();

    if player_count != players.len() {
        return TaskResult::Err(TaskError::ExpectedError(format!(
            "Task called with invalid number of players, `{}`.",
            player_count
        )));
    }

    let docker = connect_docker();

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL environment variable should be set (via .env or otherwise).");
    let pool = sqlx::SqlitePool::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    let mut path = std::env::current_dir().with_unexpected_err(|| "No current directory.")?;
    path = path.join("binaries");
    let mut containers = vec![];

    for (username, binary_hash) in players {
        let binary = Binary::get_by_username_and_hash(&username, &binary_hash, &pool)
            .await;

        if binary.is_none() {
            return TaskResult::Err(TaskError::ExpectedError(format!(
                "Task called with non-existent binary hash ({})/username ({}).",
                binary_hash, username
            )));
        }

        let binary_path = path.join(Path::new(&format!(
            "{}-{}",
            username,
            binary.unwrap().created_at
        )));

        let i = Instant::now();
        let container_id =
            create_container(&docker, binary_path.to_str().expect("binary path invalid")).await;
        let duration = Instant::now().duration_since(i);
        log::debug!("Container creation took {}ms.", duration.as_millis());

        containers.push(PlayerContainer {
            username,
            binary_hash,
            container_id,
        });
    }

    loop {
        let input = game.get_game_state();

        match input {
            GameState::Complete(scores) => {
                break;
            },
            GameState::Turn(turn_data) => {
                let player_container = &containers[turn_data.player_index];

                let r = exec_container_binary(&docker, &player_container.container_id, turn_data.stdin).await;
        
                log::info!("{:?}", r);
        
                let stdout = r.stdout();
                let card = stdout.split("\n").last().expect("stdout has no content");
                let turn_result = game.respond(card.trim());

                log::info!("{:?}", turn_result);
            }
        };
    }

    for player_container in containers {
        teardown_container(&docker, &player_container.container_id).await;
    }

    TaskResult::Ok(())
}
