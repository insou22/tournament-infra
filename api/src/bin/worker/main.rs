use tournament_api::errors::*;
use tournament_api::tasks::celery_app_factory;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info,sqlx=warn"))
        .init();
    std::env::var("DATABASE_URL")
        .expect("DATABASE_URL environment variable should be set (via .env or otherwise).");

    let app = celery_app_factory().await?;

    app.display_pretty().await;
    app.consume().await?;

    Ok(())
}
