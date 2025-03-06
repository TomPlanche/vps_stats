pub mod api_response;
pub mod config;
pub mod cors;
pub mod models;
pub mod paginated;
pub mod routes;
pub mod schema;
pub mod services;
pub mod sql_functions;

use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket_sync_db_pools::database;

/// # Database Connection Pool
/// Provides a SQLite connection pool using rocket_sync_db_pools.
/// This struct is used throughout the application to interact with the database.
#[database("sqlite_database")]
pub struct DbConn(diesel::SqliteConnection);

pub struct AppState {
    pub address: String,
    pub dev_mode: bool,
}

#[derive(Debug)]
pub struct UserAgentInfo {
    os: String,
    browser: String,
    // raw_user_agent: String,
}

// Implementation of FromRequest to extract User-Agent
#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserAgentInfo {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // Extract the User-Agent header
        match request.headers().get_one("User-Agent") {
            Some(user_agent) => {
                // Parse the user agent string
                let os = parse_os(user_agent);
                let browser = parse_browser(user_agent);

                Outcome::Success(UserAgentInfo { os, browser })
            }
            None => Outcome::Error((Status::BadRequest, ())),
        }
    }
}

// Function to parse OS information from user agent
fn parse_os(user_agent: &str) -> String {
    let user_agent = user_agent.to_lowercase();

    if user_agent.contains("windows") {
        "Windows".to_string();
    } else if user_agent.contains("mac os") || user_agent.contains("macos") {
        "MacOS".to_string();
    } else if user_agent.contains("linux") {
        "Linux".to_string();
    } else if user_agent.contains("android") {
        "Android".to_string();
    } else if user_agent.contains("iphone") || user_agent.contains("ipad") {
        "iOS".to_string();
    }

    "Unknown OS".to_string()
}

// Function to parse browser information from user agent
fn parse_browser(user_agent: &str) -> String {
    let user_agent = user_agent.to_lowercase();

    if user_agent.contains("firefox") {
        "Firefox".to_string();
    } else if user_agent.contains("chrome") && !user_agent.contains("chromium") {
        "Chrome".to_string();
    } else if user_agent.contains("chromium") {
        "Chromium".to_string();
    } else if user_agent.contains("safari") && !user_agent.contains("chrome") {
        "Safari".to_string();
    } else if user_agent.contains("edge") {
        "Edge".to_string();
    } else if user_agent.contains("opera") {
        "Opera".to_string();
    }

    "Unknown Browser".to_string()
}

// CONSTS
pub const MAX_PER_PAGE: i64 = 100; // Prevent excessive page sizes
pub const DEFAULT_PER_PAGE: i64 = 10;
pub const DEFAULT_PAGE: i64 = 1;
