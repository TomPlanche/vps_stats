use chrono::NaiveDateTime;
use diesel::{
    QueryDsl, QueryResult, RunQueryDsl,
    prelude::{Associations, Identifiable, Insertable, Queryable},
};
use serde::{Deserialize, Serialize};

use crate::paginated::{Paginate, PaginationResult};
use crate::{DbConn, models::Collector, schema::event};

#[derive(Queryable, Associations, Identifiable, Serialize, Deserialize)]
#[diesel(belongs_to(Collector, foreign_key = collector_id))]
#[diesel(table_name = event)]
#[serde(crate = "rocket::serde")]
pub struct Event {
    pub id: String,
    pub url: String,
    pub referrer: Option<String>,
    pub name: String,
    pub timestamp: NaiveDateTime,
    pub collector_id: String,
}

#[derive(Deserialize)]
pub struct EventRequest {
    pub url: String,
    pub name: String,
    pub collector_id: String,
}

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = event)]
#[serde(crate = "rocket::serde")]
pub struct NewEvent {
    pub id: String,
    pub url: String,
    pub referrer: Option<String>,
    pub name: String,
    pub timestamp: NaiveDateTime,
    pub collector_id: String,
}

#[derive(Deserialize)]
pub struct EventQuery {
    url: String,
    referrer: Option<String>,
    name: String,
    collector_id: String,
}

impl Event {
    /// # `insert`
    /// Inserts a new event into the database.
    ///
    /// ## Arguments
    /// * `event` - Event data to insert
    /// * `conn` - Database connection
    ///
    /// ## Returns
    /// * `QueryResult<String>` - Result of the insert operation
    pub async fn insert(event: NewEvent, conn: &DbConn) -> QueryResult<String> {
        conn.run(|c| {
            diesel::insert_into(event::table)
                .values(&event)
                .execute(c)?;

            Ok(event.id)
        })
        .await
    }

    /// # `all`
    /// Retrieves all `transit_stops` from the database.
    ///
    /// ## Arguments
    /// * `page` - The page number
    /// * `per_page` - The number of items per page
    /// * `conn` - Database connection
    ///
    /// ## Errors
    /// If the `transit_stops` cannot be retrieved
    pub async fn all(
        page: i64,
        per_page: i64,
        conn: &DbConn,
    ) -> QueryResult<PaginationResult<Event>> {
        conn.run(move |c| {
            event::table
                .order(event::id)
                .paginate(page)
                .per_page(per_page)
                .load_and_count_pages(c)
        })
        .await
    }
}

// /// # Record Event
// /// Handles POST requests to record a new event
// ///
// /// ## Arguments
// /// * `conn` - Database connection
// /// * `is_development` - Development mode flag
// /// * `event_data` - Event data from request
// #[post("/events", data = "<event_data>")]
// pub async fn record_event(conn: DbConn, event_data: Json<EventQuery>) -> Json<serde_json::Value> {
//     let localhost_regex = Regex::new(r"http://(127\.0\.0\.1|localhost|0\.0\.0\.0|\[::1\])(:\d+)?")
//         .expect("Invalid regex pattern");

//     // Block local requests in production
//     if localhost_regex.is_match(&event_data.url) {
//         return ApiResponse::bad_request("Local URLs are not allowed in production");
//     }

//     // Clean URL
//     let clean_url = match Url::parse(&event_data.url) {
//         Ok(mut url) => {
//             url.set_query(None);
//             url.to_string().trim_end_matches('/').to_string()
//         }
//         Err(_) => event_data.url.trim_end_matches('/').to_string(),
//     };

//     let new_event = NewEvent {
//         id: Ulid::new().to_string(),
//         url: clean_url,
//         referrer: None, // You might want to add this to EventRequest
//         name: event_data.name.clone(),
//         timestamp: Utc::now().naive_utc(),
//         collector_id: event_data.collector_id.clone(),
//     };

//     // Use connection to insert event
//     match conn
//         .run(move |c| {
//             diesel::insert_into(event::table)
//                 .values(&new_event)
//                 .execute(c)
//         })
//         .await
//     {
//         Ok(_) => ApiResponse::created(serde_json::json!({
//             "message": "Event recorded successfully"
//         })),
//         Err(e) => ApiResponse::internal_error(&format!("Failed to record event: {}", e)),
//     }
// }

// /// # Retrieve Events
// /// Handles GET requests to fetch all events
// ///
// /// ## Arguments
// /// * `conn` - Database connection
// #[get("/events")]
// pub async fn retrieve_events(conn: DbConn) -> Json<serde_json::Value> {
//     match conn.run(|c| event::table.load::<Event>(c)).await {
//         Ok(events_list) => ApiResponse::success(events_list),
//         Err(e) => ApiResponse::internal_error(&format!("Error retrieving events: {}", e)),
//     }
// }
