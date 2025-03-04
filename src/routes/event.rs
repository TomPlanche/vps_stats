use crate::{ApiResponse, DbConn, models::Event, paginated::set_pagination_defaults};
use chrono::Utc;
use regex::Regex;
use rocket::{get, post, serde::json::Json};
use serde::Deserialize;
use serde_json::{Value, json};
use ulid::Ulid;
use url::Url;

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct EventQuery {
    url: String,
    referrer: Option<String>,
    name: String,
    collector_id: String,
}

/// # `event_insert`
/// Handles POST requests to insert a new event.
///
/// ## Arguments
/// * `event_data` - Event data from request
/// * `conn` - Database connection
///
/// ## Panics
/// If the regex pattern is invalid.
#[post("/", data = "<event_data>")]
pub async fn event_insert(event_data: Json<EventQuery>, conn: DbConn) -> Json<serde_json::Value> {
    let localhost_regex = Regex::new(r"http://(127\.0\.0\.1|localhost|0\.0\.0\.0|\[::1\])(:\d+)?")
        .expect("Invalid regex pattern");

    // Block local requests in production
    if localhost_regex.is_match(&event_data.url) {
        return ApiResponse::bad_request("Local URLs are not allowed in production");
    }

    // Clean URL
    let clean_url = match Url::parse(&event_data.url) {
        Ok(mut url) => {
            url.set_query(None);
            url.to_string().trim_end_matches('/').to_string()
        }
        Err(_) => event_data.url.trim_end_matches('/').to_string(),
    };

    let new_event = Event {
        id: Ulid::new().to_string(),
        url: clean_url,
        referrer: None, // You might want to add this to EventRequest
        name: event_data.name.clone(),
        timestamp: Utc::now().naive_utc(),
        collector_id: event_data.collector_id.clone(),
    };

    // Use connection to insert event
    match Event::insert(new_event, &conn).await {
        Ok(id) => ApiResponse::created(serde_json::json!({
            "message": &format!("Event #{id} recorded successfully")
        })),
        Err(e) => ApiResponse::internal_error(&format!("Failed to record event: {e}")),
    }
}

/// # Retrieve Events
/// Handles GET requests to fetch all events.
///
/// ## Arguments
/// * `page` - Page number for pagination
/// * `per_page` - Number of items per page
/// * `conn` - Database connection
///
/// ## Returns
/// * `Json<Value>` - JSON response containing events data
#[get("/?<page>&<per_page>")]
pub async fn event_get(page: Option<i64>, per_page: Option<i64>, conn: DbConn) -> Json<Value> {
    let (page, per_page) = set_pagination_defaults(page, per_page);

    match Event::all(page, per_page, &conn).await {
        Ok(events_list) => ApiResponse::success(json!({
            "events": events_list
        })),
        Err(e) => ApiResponse::internal_error(&format!("Error retrieving events: {e}")),
    }
}
