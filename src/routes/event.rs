use crate::api_response::ApiResponse;
use crate::{
    DbConn,
    models::{Event, EventQuery},
    paginated::set_pagination_defaults,
};
use regex::Regex;
use rocket::{get, post, serde::json::Json};
use serde_json::{Value, json};
use url::Url;

/// # `event_insert`
/// Handles POST requests to insert a new event.
///
/// ## Arguments
/// * `event_data` - Event data from request
/// * `conn` - Database connection
///
/// ## Panics
/// If the regex pattern is invalid.
#[post("/", format = "application/json", data = "<event_data>")]
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

    let mut new_event = event_data.into_inner(); // from Json<EventQuery> to EventQuery
    new_event.url = clean_url;

    let new_event: Event = new_event.into(); // from EventQuery to Event, `: Event` not needed

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
