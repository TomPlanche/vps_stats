use chrono::NaiveDateTime;
use diesel::{
    QueryResult, RunQueryDsl,
    prelude::{Identifiable, Insertable, Queryable},
};
use serde::{Deserialize, Serialize};
use ulid::Ulid;

use crate::{DbConn, schema::collector};

use super::Event;

#[derive(Queryable, Insertable, Identifiable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = collector)]
#[serde(crate = "rocket::serde")]
pub struct Collector {
    pub id: String,
    pub origin: String,
    pub city_id: i32,
    pub os: Option<String>,
    pub browser: Option<String>,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize)]
pub struct CollectorWithEvents {
    pub collector: Collector,
    pub events: Vec<Event>,
}

#[derive(Deserialize)]
pub struct CollectorQuery {
    pub origin: String,
    pub city_id: i32,
    pub os: Option<String>,
    pub browser: Option<String>,
}

impl From<CollectorQuery> for Collector {
    fn from(query: CollectorQuery) -> Self {
        Collector {
            id: Ulid::new().to_string(),
            origin: query.origin,
            city_id: query.city_id,
            os: query.os,
            browser: query.browser,
            created_at: None,
        }
    }
}

impl Collector {
    /// # `insert`
    /// Inserts a new `Collector` into the database.
    ///
    /// ## Arguments
    /// * `collector` - Collector data to insert
    /// * `conn` - Database connection
    ///
    /// ## Errors
    /// If the `Collector` cannot be inserted into the database.
    ///
    /// ## Returns
    /// * `QueryResult<String>` - Id of the inserted `Collector`
    pub async fn insert(collector: Collector, conn: &DbConn) -> QueryResult<String> {
        conn.run(|c| {
            diesel::insert_into(collector::table)
                .values(&collector)
                .execute(c)?;

            Ok(collector.id)
        })
        .await
    }
}
