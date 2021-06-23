use crate::MainDbConn;
use crate::{
    models::{Ranking, User},
    schema::{rankings, users},
};
use diesel::prelude::*;
use rocket::response::status::NotFound;
use rocket::serde::json::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct ProfileResponse {
    username: String,
    display_name: String,
    current_elo: Option<i32>,
}

#[get("/user/<username>")]
pub async fn user_profile(
    conn: MainDbConn,
    username: String,
) -> Result<Json<ProfileResponse>, NotFound<()>> {
    let user = conn
        .run(|c| {
            users::table
                .filter(users::columns::username.eq(username))
                .first::<User>(c)
                .optional()
                .expect("user find failed")
        })
        .await;

    if user.is_none() {
        return Err(NotFound(()));
    }

    let user = user.unwrap();

    let user_id = user.id;

    let current_ranking = conn
        .run(move |c| {
            rankings::table
                .filter(rankings::columns::user_id.eq(user_id))
                .filter(
                    rankings::columns::tournament_id.eq(1i32), // TODO: Get this from the config file.
                )
                .first::<Ranking>(c)
                .optional()
                .expect("ranking find failed")
        })
        .await;

    let current_elo = current_ranking.and_then(|r| Some(r.elo));

    return Ok(Json(ProfileResponse {
        username: user.username,
        display_name: user.display_name,
        current_elo,
    }));
}
