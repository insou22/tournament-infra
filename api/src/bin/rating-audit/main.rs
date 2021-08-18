use futures_util::TryStreamExt;
use tournament_api::errors::*;
use tournament_api::models::game::Game;
use tournament_api::models::ranking::Ranking;
use tournament_api::rating::get_rating_change;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info,sqlx=warn"))
        .init();
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL environment variable should be set (via .env or otherwise).");

    let pool = sqlx::SqlitePool::connect(&database_url).await?;

    let (default_mu, default_sigma) = {
        let r = bbt::Rating::default();
        (r.mu(), r.sigma())
    };

    sqlx::query!(
        "UPDATE rankings SET rating_mu=?, rating_sigma=?",
        default_mu,
        default_sigma
    )
    .execute(&pool)
    .await?;

    let mut games_stream =
        sqlx::query_as!(Game, "SELECT * FROM games ORDER BY created_at ASC").fetch(&pool);

    while let Some(game) = games_stream.try_next().await? {
        let mut players = vec![];
        for player in game.get_players(&pool).await {
            let ranking = sqlx::query_as!(
                Ranking,
                r#"SELECT id, user_id, tournament_id, rating_mu AS "rating_mu: f64", rating_sigma AS "rating_sigma: f64" FROM rankings
                WHERE user_id=(SELECT id FROM users WHERE username=?) AND tournament_id=?"#,
                player.username,
                game.tournament_id
            )
            .fetch_one(&pool)
            .await?;
            players.push((
                player.username,
                match player.result.as_str() {
                    "won" => 2,
                    "drew" => 1,
                    "lost" => 0,
                    _ => unreachable!()
                },
                bbt::Rating::new(ranking.rating_mu, ranking.rating_sigma),
            ))
        }

        let players = get_rating_change(players)?;

        for (username, _, rating, new_rating) in players {
            let new_rating_mu = new_rating.mu();
            let new_rating_sigma = new_rating.sigma();
            let rating_mu = rating.mu();
            let rating_sigma = rating.sigma();
            let rating_change_mu = rating_mu - new_rating_mu;
            let rating_change_sigma = rating_sigma - new_rating_sigma;

            sqlx::query!(
                "UPDATE players SET rating_mu_before_game=?, rating_sigma_before_game=?, rating_mu_change=?, rating_sigma_change=?
                WHERE game_id=? AND user_id=(SELECT id FROM users WHERE username=?)",
                rating_mu,
                rating_sigma,
                rating_change_mu,
                rating_change_sigma,
                game.id,
                username
            )
            .execute(&pool)
            .await?;

            sqlx::query!(
                "UPDATE rankings SET rating_mu=?, rating_sigma=?
                WHERE user_id=(SELECT id FROM users WHERE username=?) AND tournament_id=?",
                new_rating_mu,
                new_rating_sigma,
                username,
                game.tournament_id
            )
            .execute(&pool)
            .await?;
        }
    }

    Ok(())
}
