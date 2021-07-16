use rocket::{
    fairing::{Fairing, Info, Kind},
    http::{Header},
    Request, Response
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

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        // TODO: Revisit CORS down when we have a domain
        response.set_header(Header::new(
            "Access-Control-Allow-Origin",
            "http://localhost:8080",
        ));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "GET, POST, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "Content-Type"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}
