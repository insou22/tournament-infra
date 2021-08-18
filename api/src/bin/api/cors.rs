use rocket::{
    fairing::{Fairing, Info, Kind},
    http::{Header, Method, Status},
    Data, Request, Response,
};

pub fn fairing() -> Cors {
    Cors
}

pub struct Cors;

#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "CORS preflights and headers",
            kind: Kind::Ignite | Kind::Response,
        }
    }

    async fn on_request(&self, _req: &mut Request<'_>, _data: &mut Data<'_>) {}

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        let config = request
            .guard::<&rocket::State<tournament_api::config::Config>>()
            .await
            .expect("config fetch failed in CORS fairing");
        // TODO: Revisit CORS down when we have a domain
        response.set_header(Header::new(
            "Access-Control-Allow-Origin",
            &config.inner().webapp_url,
        ));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "GET, POST, PUT, PATCH, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "Content-Type"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));

        if request.method() == Method::Options && request.route().is_none() {
            response.set_status(Status::NoContent);
        }
    }
}
