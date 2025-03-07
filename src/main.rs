use crate::api_response::ApiResponse;
use rocket::{
    Request, catch, catchers,
    figment::Figment,
    http::Status,
    launch, routes,
    serde::json::{Json, Value, json},
};
use website_stats::{
    AppState, DbConn,
    config::AppConfig,
    cors::Cors,
    routes::{
        city::{city_get, city_insert},
        collector::stats_js,
        event::{event_get, event_insert},
        session::{get_map_data, get_sessions},
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

    let app_config = AppConfig::new();
    let dev_mode = app_config.dev;
    let address = app_config.address.clone();
    let figment: Figment = app_config.into();
    let app_state = AppState { address, dev_mode };

    rocket::build()
        .configure(figment)
        .attach(DbConn::fairing())
        .attach(Cors)
        .manage(app_state)
        .register("/", catchers![default_catcher])
        .mount("/", routes![root])
        .mount("/event", routes![event_insert, event_get])
        .mount("/city", routes![city_insert, city_get])
        .mount("/session", routes![get_sessions, get_map_data])
        .mount("/stats.js", routes![stats_js])
}
