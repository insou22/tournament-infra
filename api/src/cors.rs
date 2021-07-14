use rocket::{
    Build, Request, Response, Rocket,
    fairing::{
        self, Fairing, Info, Kind
    },
    http::{
        Header, Method
    }
};

pub fn fairing() -> Cors {
    Cors
}

pub struct Cors;

#[options("/")]
fn cors_handler() {}

#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "CORS preflights and headers",
            kind: Kind::Ignite | Kind::Response,
        }
    }

    // Calling rocket.mount() on every route breaks dynamic routing!
    // async fn on_ignite(&self, mut rocket: Rocket<Build>) -> fairing::Result {
    //     let routes = rocket.routes()
    //         .filter(|route| route.method != Method::Options)
    //         .map(|route| route.uri.as_str().to_string())
    //         .collect::<Vec<_>>();
        
    //     for route in routes {
    //         rocket = rocket.mount(route, routes![cors_handler]);
    //     }

    //     Ok(rocket)
    // }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        // TODO: Revisit CORS down when we have a domain
        response.set_header(Header::new("Access-Control-Allow-Origin", "http://localhost:8080"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "GET, POST, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "Content-Type"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}