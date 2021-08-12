use crate::game::Game;
use crate::games::create_game_by_name;
use crate::isolator::{
    connect_docker, create_container, exec_container_binary, teardown_container,
};
use celery::prelude::*;
use sqlx::Connection;
use std::path::Path;
use std::time::Instant;

struct PlayerContainer {
    username: String,
    binary_hash: String,
    container_id: String,
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

    let (game, player_count) = game_details.unwrap();

    if player_count as usize != players.len() {
        return TaskResult::Err(TaskError::ExpectedError(format!(
            "Task called with invalid number of players, `{}`.",
            player_count
        )));
    }

    let docker = connect_docker();

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL environment variable should be set (via .env or otherwise).");
    let mut conn = sqlx::SqliteConnection::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    let mut path = std::env::current_dir().with_unexpected_err(|| "No current directory.")?;
    path = path.join("binaries");
    let mut containers = vec![];

    for (username, binary_hash) in players {
        let created_at_record = sqlx::query!(
            "SELECT created_at FROM binaries
            WHERE user_id=(SELECT id FROM users WHERE username=?) AND hash=?",
            username,
            binary_hash
        )
        .fetch_optional(&mut conn)
        .await
        .expect("binary find failed during task.");

        if created_at_record.is_none() {
            return TaskResult::Err(TaskError::ExpectedError(format!(
                "Task called with non-existent binary hash ({})/username ({}).",
                binary_hash, username
            )));
        }

        let binary_path = path.join(Path::new(&format!(
            "{}-{}",
            username,
            created_at_record.unwrap().created_at
        )));

        let i = Instant::now();
        let container_id =
            create_container(&docker, binary_path.to_str().expect("binary path invalid")).await;
        let duration = Instant::now().duration_since(i);
        log::debug!("Container creation took {}ms.", duration.as_millis());

        let r = exec_container_binary(&docker, &container_id, "".to_owned()).await;

        log::info!("{:?}", r);

        teardown_container(&docker, &container_id).await;

        containers.push(PlayerContainer {
            username,
            binary_hash,
            container_id,
        });
    }

    TaskResult::Ok(())
}
