use chrono::{Duration, Utc};
use diesel::{
    BelongingToDsl, ExpressionMethods, GroupedBy, QueryDsl, QueryResult, RunQueryDsl,
    prelude::QueryableByName,
    result::Error,
    sql_types::{BigInt, Float, Text, Timestamp},
    sqlite::Sqlite,
};
use serde::{Deserialize, Serialize};

use crate::{
    DbConn,
    models::{self, CollectorWithEvents, Event},
    schema::{collector, event},
};

/// # `retrieve_sessions`
/// Retrieve the last 30 recent visitor sessions.
///
/// ## Arguments
/// * `conn` - A database connection.
///
/// ## Errors
/// * `Error::NotFound` - If no collectors are found.
///
/// ## Returns
/// * `QueryResult<Vec<CollectorWithEvents>>` - The result of the query.
pub async fn retrieve_sessions(conn: &DbConn) -> QueryResult<Vec<CollectorWithEvents>> {
    let last_30_collectors = match conn
        .run(move |c| {
            collector::table
                .order(collector::created_at.desc())
                .limit(30)
                .load::<models::Collector>(c)
        })
        .await
    {
        Ok(collectors) => collectors,
        Err(err) => {
            eprintln!("Error loading collectors: {err:?}");
            return Err(Error::NotFound);
        }
    };

    if last_30_collectors.is_empty() {
        return Ok(Vec::new());
    }

    let collectors_ids: Vec<String> = last_30_collectors.iter().map(|c| c.id.clone()).collect();

    let last_30_collectors_clone = last_30_collectors.clone();
    let events_from_collectors = match conn
        .run(move |c| {
            Event::belonging_to(&last_30_collectors_clone)
                .filter(event::collector_id.eq_any(collectors_ids))
                .order(event::created_at.desc())
                .limit(30)
                .load::<models::Event>(c)
        })
        .await
    {
        Ok(events) => events,
        Err(err) => {
            eprintln!("Error loading events: {err:?}");
            return Err(Error::NotFound);
        }
    }
    .grouped_by(&last_30_collectors);

    // Remove collectors with no events
    let collectors_with_events: Vec<CollectorWithEvents> = last_30_collectors
        .into_iter()
        .zip(events_from_collectors)
        .filter(|(_, events)| !events.is_empty())
        .map(|(collector, events)| CollectorWithEvents { collector, events })
        .collect();

    Ok(collectors_with_events)
}

#[derive(Serialize, Deserialize)]
pub struct CityCollectorCount {
    pub lat: f32,
    pub lng: f32,
    pub size: f32,
    pub color: String,
    pub city: String,
}
#[derive(QueryableByName)]
#[diesel(check_for_backend(Sqlite))]
pub struct CityCount {
    #[diesel(sql_type = Text)]
    pub name: String,
    #[diesel(sql_type = Float)]
    pub latitude: f32,
    #[diesel(sql_type = Float)]
    pub longitude: f32,
    #[diesel(sql_type = BigInt)]
    pub count: i64,
}

/// # `map`
/// Retrieves and processes visitor location data from the past 7 days for map visualization.
///
/// This function:
/// 1. Queries the database for collector counts grouped by city
/// 2. Calculates relative sizes for visual representation
/// 3. Formats the data for map display
///
/// ## Arguments
/// * `conn` - A database connection
///
/// ## Errors
/// * `Error::NotFound` - If the database query fails
///
/// ## Returns
/// * `QueryResult<Vec<CityCollectorCount>>` - A vector of city data points containing:
///   - Geographic coordinates (lat/lng)
///   - Relative size (normalized between 0 and 1)
///   - City name
///   - Display color
pub async fn map(conn: &DbConn) -> QueryResult<Vec<CityCollectorCount>> {
    let seven_days_ago = Utc::now().naive_utc() - Duration::days(7);

    let query = "
        SELECT ci.name, ci.latitude, ci.longitude, COUNT(*) AS count
        FROM collector co
        JOIN city ci ON ci.id = co.city_id
        WHERE co.created_at >= ?
        GROUP BY ci.name, ci.latitude, ci.longitude;
    ";

    let results: Vec<CityCount> = match conn
        .run(move |c| {
            diesel::sql_query(query)
                .bind::<Timestamp, _>(seven_days_ago)
                .load::<CityCount>(c)
        })
        .await
    {
        Ok(results) => results,
        Err(e) => {
            eprintln!("Error querying city collector counts: {e:?}");
            return Err(Error::NotFound);
        }
    };

    // Find the maximum count to normalize sizes
    let max_count = results.iter().map(|c| c.count).max().unwrap_or(1);
    let mut city_counts: Vec<CityCollectorCount> = Vec::new();

    for city_count in results {
        #[allow(clippy::cast_precision_loss)]
        let relative_size = (city_count.count / max_count) as f32;
        city_counts.push(CityCollectorCount {
            city: city_count.name,
            lat: city_count.latitude,
            lng: city_count.longitude,
            size: relative_size,
            color: "#fa4f33".to_string(),
        });
    }

    Ok(city_counts)
}
