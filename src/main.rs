use crate::api_response::ApiResponse;
use rocket::{
    Request, catch, catchers,
    figment::Figment,
    http::Status,
    launch, routes,
    serde::json::{Json, Value, json},
};
use website_stats::{
    DbConn,
    config::AppConfig,
    cors::Cors,
    routes::{
        city::{city_get, city_insert},
        event::{event_get, event_insert},
    },
};

mod api_response;
mod cors;

#[catch(default)]
fn default_catcher(status: Status, _req: &Request) -> Json<Value> {
    ApiResponse::error(status, &status.to_string())
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
    dotenv::dotenv().ok();

    let figment: Figment = AppConfig::new().into();

    rocket::build()
        .configure(figment)
        .attach(DbConn::fairing())
        .attach(Cors)
        .manage(true)
        .register("/", catchers![default_catcher])
        .mount("/", routes![root])
        .mount("/event", routes![event_insert, event_get])
        .mount("/city", routes![city_insert, city_get])
}
