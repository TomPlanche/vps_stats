use rocket::{
    fairing::{Fairing, Info, Kind},
    http::Header,
};

/// CORS Configuration
/// Implements CORS headers for the application
pub struct Cors;

#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "Cross-Origin-Resource-Sharing Fairing",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(
        &self,
        request: &'r rocket::Request<'_>,
        response: &mut rocket::Response<'r>,
    ) {
        println!("CORS Fairing");
        println!(
            "Request came from: {:?}",
            request.headers().get_one("Origin")
        );

        let allowed_origin = request
            .headers()
            .get_one("Origin")
            .unwrap_or("http://localhost:5173");

        response.set_header(Header::new("Access-Control-Allow-Origin", allowed_origin));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "GET, POST",
        ));
        response.set_header(Header::new(
            "Access-Control-Allow-Headers",
            "Content-Type, Authorization",
        ));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));

        if request.method() == rocket::http::Method::Options {
            response.set_header(Header::new("Access-Control-Max-Age", "86400"));
            response.set_status(rocket::http::Status::NoContent);
        }
    }
}
