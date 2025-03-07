use rocket::{
    Request,
    fairing::{Fairing, Info, Kind},
};

use crate::logger::Logger;

pub struct RequestLogger;

#[rocket::async_trait]
impl Fairing for RequestLogger {
    fn info(&self) -> Info {
        Info {
            name: "Request Logger",
            kind: Kind::Request,
        }
    }

    async fn on_request(&self, request: &mut Request<'_>, _: &mut rocket::Data<'_>) {
        let method = request.method();
        let uri = request.uri();
        let remote_addr = request
            .remote()
            .map_or(String::from("Unknown"), |addr| addr.ip().to_string());

        let user_agent = request.headers().get_one("User-Agent").unwrap_or("Unknown");

        Logger::info(
            "Request",
            &format!("{} {} from {} ({})", method, uri, remote_addr, user_agent),
        );
    }
}
