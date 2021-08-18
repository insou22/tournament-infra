use tournament_api::errors::*;
use tournament_api::models::game::Game;
use futures_util::TryStreamExt;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info,sqlx=warn"))
        .init();
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL environment variable should be set (via .env or otherwise).");

    let pool = sqlx::SqlitePool::connect(&database_url)
        .await?;

    let mut games_stream = sqlx::query_as!(Game, "SELECT * FROM games ORDER BY created_at ASC").fetch(&pool);

    while let Some(game) = games_stream.try_next().await? {
        let players = game.get_players(&pool).await;

        for player in players {
            println!("{}", player.username);
        }
    }

    Ok(())
}
