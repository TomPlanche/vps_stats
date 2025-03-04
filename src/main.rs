use rocket::{
    Request, catch, catchers,
    fairing::{Fairing, Info, Kind},
    http::{Header, Status},
    launch, routes,
    serde::json::{Json, Value, json},
};
use serde::Serialize;
use website_stats::{
    ApiResponse, DbConn,
    routes::{
        city::{city_get, city_insert},
        event::{event_get, event_insert},
    },
};

/// # CORS Configuration
/// Implements CORS (Cross-Origin Resource Sharing) headers for the application.
/// Allows requests from localhost:5173 during development.
pub struct Cors;

#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "Cross-Origin-Resource-Sharing Fairing",
            kind: Kind::Response,
        }
    }

    /// # `on_response`
    /// Sets CORS headers for the response.
    ///
    /// Sets the following headers:
    /// - Access-Control-Allow-Origin: http://localhost:3000 # port of the SvelteKit app
    /// - Access-Control-Allow-Methods: GET, POST, PUT, DELETE, OPTIONS
    /// - Access-Control-Allow-Headers: Content-Type
    /// - Access-Control-Allow-Credentials: true
    async fn on_response<'r>(
        &self,
        _request: &'r rocket::Request<'_>,
        response: &mut rocket::Response<'r>,
    ) {
        response.set_header(Header::new(
            "Access-Control-Allow-Origin",
            "http://localhost:3000",
        ));
        response.set_header(Header::new(
            "Access-Control-Allow-Origin",
            "http://localhost:5173",
        ));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "GET, POST, PUT, DELETE, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "Content-Type"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[derive(Serialize)]
struct ErrorResponse {
    error: ErrorDetail,
}

#[derive(Serialize)]
struct ErrorDetail {
    code: u16,
    message: String,
}

#[catch(default)]
fn default_catcher(status: Status, _req: &Request) -> Json<ErrorResponse> {
    Json(ErrorResponse {
        error: ErrorDetail {
            code: status.code,
            message: status.to_string(),
        },
    })
}

/// # `root`
/// Handles GET requests to the root path ("/").
/// Serves as a simple health check endpoint.
///
/// ## Returns
/// A static string greeting message
#[rocket::get("/")]
fn root() -> Json<Value> {
    ApiResponse::success(json!({
        "message": "Hello, visitors!",
    }))
}

/// # `rocket`
/// Configures and launches the Rocket application.
/// Sets up database connection, runs migrations, configures CORS, and mounts routes.
///
/// ## Returns
/// The configured Rocket instance
#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(DbConn::fairing())
        .attach(Cors)
        .manage(true)
        .register("/", catchers![default_catcher])
        .mount("/", routes![root])
        .mount("/event", routes![event_insert, event_get])
        .mount("/city", routes![city_insert, city_get])
}
