use crate::errors::*;
use rocket::{
    http::Status, outcome::{try_outcome, IntoOutcome}, request::FromRequest, request::Outcome, Request, State,
};
use sqlx::Acquire;

// pub async fn get_connection(
//     pool_state: &rocket::State<sqlx::SqlitePool>,
// ) -> sqlx::pool::PoolConnection<sqlx::Sqlite> {
//     pool_state
//         .inner()
//         .acquire()
//         .await
//         .expect("could not acquire pool connection")
// }

pub struct DBTransaction<'r>(&'r mut sqlx::SqliteConnection);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for DBTransaction<'r> {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, ()> {
        let pool_state = try_outcome!(request.guard::<&State<sqlx::SqlitePool>>().await);

        let connection = request.local_cache_async(async {
            let pool_state = request.guard::<&State<sqlx::SqlitePool>>().await.success_or(Err(Outcome::Failure((Status::InternalServerError, ()))))?;

            match pool_state.inner().acquire().await {
                Ok(c) => Ok(&c),
                Err(_) => Err(Outcome::Failure((Status::InternalServerError, ())))
            }
        }).await;

        let connection = match pool_state.inner().acquire().await {
            Ok(c) => c,
            Err(_) => return Outcome::Failure((Status::InternalServerError, ()))
        };

        match connection {
            Ok(conn) => Outcome::Success(DBTransaction(&mut conn.begin().await)),
            Err(_) => Outcome::Failure((Status::InternalServerError, ()))
        }
    }
}
