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
        collector::collector_stats_js,
        event::{event_get, event_insert},
        session::{session_get_map_data, session_get_sessions},
        summary::{
            summary_get_browsers, summary_get_events, summary_get_five_minutes, summary_get_hourly,
            summary_get_os_browsers, summary_get_percentages, summary_get_referrers,
        },
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
        .mount("/city", routes![city_insert, city_get])
        .mount("/event", routes![event_insert, event_get])
        .mount(
            "/session",
            routes![session_get_sessions, session_get_map_data],
        )
        .mount(
            "/summary",
            routes![
                summary_get_browsers,
                summary_get_events,
                summary_get_five_minutes,
                summary_get_hourly,
                summary_get_os_browsers,
                summary_get_percentages,
                summary_get_referrers
            ],
        )
        .mount("/stats.js", routes![collector_stats_js])
}
