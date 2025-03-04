use chrono::{NaiveDateTime, Utc};
use diesel::{
    QueryDsl, QueryResult, RunQueryDsl,
    prelude::{Associations, Identifiable, Insertable, Queryable},
};
use serde::{Deserialize, Serialize};
use ulid::Ulid;

use crate::paginated::{Paginate, PaginationResult};
use crate::{DbConn, models::Collector, schema::event};

#[derive(Associations, Deserialize, Identifiable, Insertable, Queryable, Serialize)]
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
#[allow(dead_code)]
pub struct EventQuery {
    pub url: String,
    pub referrer: Option<String>,
    pub name: String,
    pub collector_id: String,
}

impl From<EventQuery> for Event {
    fn from(query: EventQuery) -> Self {
        Event {
            id: Ulid::new().to_string(),
            url: query.url,
            referrer: query.referrer,
            name: query.name,
            timestamp: Utc::now().naive_utc(),
            collector_id: query.collector_id,
        }
    }
}

impl Event {
    /// # `insert`
    /// Inserts a new event into the database.
    ///
    /// ## Arguments
    /// * `event` - Event data to insert
    /// * `conn` - Database connection
    ///
    /// ## Errors
    /// If the event cannot be inserted into the database.
    ///
    /// ## Returns
    /// * `QueryResult<String>` - Result of the insert operation
    pub async fn insert(event: Event, conn: &DbConn) -> QueryResult<String> {
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
