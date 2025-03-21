use rocket::{
    Request,
    fairing::{Fairing, Info, Kind},
    Response,
};

use crate::logger::Logger;

pub struct RequestLogger;

#[rocket::async_trait]
impl Fairing for RequestLogger {
    fn info(&self) -> Info {
        Info {
            name: "Request Logger",
            kind: Kind::Request | Kind::Response,
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
            &format!("{method} {uri} from {remote_addr} ({user_agent})"),
        );
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        let method = request.method();
        let uri = request.uri();
        let status = response.status();
        
        Logger::info(
            "Response",
            &format!("{method} {uri} responded with status {status}"),
        );
    }
}
