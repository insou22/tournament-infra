use celery::prelude::*;
use tournament_api::errors::*;
use tournament_api::tasks::play;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info,sqlx=warn"))
        .init();
    std::env::var("DATABASE_URL")
        .expect("DATABASE_URL environment variable should be set (via .env or otherwise).");
    std::env::var("REDIS_URL")
        .expect("REDIS_URL environment variable should be set (via .env or otherwise).");

    let app = celery::app!(
        broker = RedisBroker { std::env::var("REDIS_URL").unwrap() },
        tasks = [play],
        task_routes = [],
        task_retry_for_unexpected = false
    )
    .await?;

    app.send_task(play::new(
        "round-1".to_owned(),
        vec![
            ("z5361056".to_owned(), "46bf1c1".to_owned()),
            ("chicken".to_owned(), "17f16d4".to_owned()),
        ],
    ))
    .await?;

    app.display_pretty().await;
    app.consume().await?;

    Ok(())
}
