use chrono::{Duration, NaiveDateTime, Utc};
use diesel::{
    QueryResult, RunQueryDsl,
    prelude::QueryableByName,
    result::Error,
    sql_query,
    sql_types::{BigInt, Integer, Text, Timestamp},
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::DbConn;

#[derive(QueryableByName, Debug, Serialize)]
pub struct FiveMinuteEventSummary {
    #[diesel(sql_type = Text)]
    pub interval: String,
    #[diesel(sql_type = BigInt)]
    pub count: i64,
}

/// # `five_minutes`
/// Retrieves a minute-by-minute summary of events from the database over the past 24 hours.
///
/// ## Arguments
/// * `conn` - A database connection.
///
/// ## Errors
/// * `Error::NotFound` - If the query fails.
///
/// ## Returns
/// `QueryResult<Vec<FiveMinuteEventSummary>>` containing the summary data.
pub async fn five_minutes(conn: &DbConn) -> QueryResult<Vec<FiveMinuteEventSummary>> {
    let start_time = Utc::now().naive_utc() - Duration::days(1);

    let sql = "
        SELECT STRFTIME('%Y-%m-%d %H:%M:00', created_at) AS interval, COUNT(*) AS count
        FROM event
        WHERE event.created_at > ?
        GROUP BY STRFTIME('%Y-%m-%d %H:%M', created_at)
        ORDER BY interval;
    ";

    match conn
        .run(move |c| {
            diesel::sql_query(sql)
                .bind::<Timestamp, _>(start_time)
                .load::<FiveMinuteEventSummary>(c)
        })
        .await
    {
        Ok(query) => Ok(query),
        Err(e) => {
            eprintln!("Failed to load five-minute event summary: {e}");
            Err(Error::NotFound)
        }
    }
}

#[derive(QueryableByName, Debug, Serialize)]
pub struct EventCounts {
    #[diesel(sql_type = BigInt)]
    pub sessions_in_last_twenty_four_hours: i64,
    #[diesel(sql_type = BigInt)]
    pub events_in_last_twenty_four_hours: i64,
    #[diesel(sql_type = BigInt)]
    pub events_in_last_hour: i64,
    #[diesel(sql_type = BigInt)]
    pub events_in_last_five_minutes: i64,
}

/// # `events`
/// Retrieves summary statistics of session and event counts over different time intervals (24 hours, 1 hour, 5 minutes).
///
/// ## Arguments
/// * `conn` - A database connection.
///
/// ## Errors
/// * `Error::NotFound` - If the query fails.
///
/// ## Returns
/// `QueryResult<Vec<EventSummary>>` containing the summary data.
pub async fn events(conn: &DbConn) -> QueryResult<Vec<EventCounts>> {
    let query = "SELECT \
        (SELECT COUNT(*) FROM collector WHERE created_at >= datetime('now', '-24 hours')) AS sessions_in_last_twenty_four_hours, \
        (SELECT COUNT(*) FROM event WHERE created_at >= datetime('now', '-24 hours')) AS events_in_last_twenty_four_hours, \
        (SELECT COUNT(*) FROM event WHERE created_at >= datetime('now', '-5 minutes')) AS events_in_last_five_minutes, \
        (SELECT COUNT(*) FROM event WHERE created_at >= datetime('now', '-1 hour')) AS events_in_last_hour";

    match conn
        .run(move |c| sql_query(query).load::<EventCounts>(c))
        .await
    {
        Ok(query) => Ok(query),
        Err(e) => {
            eprintln!("Failed to load five-minute event summary: {e}");
            Err(Error::NotFound)
        }
    }
}

#[derive(QueryableByName, Serialize)]
pub struct HourlyEventSummary {
    #[diesel(sql_type = Timestamp)]
    hour: NaiveDateTime,
    #[diesel(sql_type = Integer)]
    count: i32,
}

/// # `hourly`
/// Retrieves an hourly breakdown of events from the database over the past 24 hours.
///
/// ## Arguments
/// * `conn` - A database connection.
///
/// ## Errors
/// * `Error::NotFound` - If the query fails.
///
/// ## Returns
/// `QueryResult<Vec<HourlyEventSummary>>` containing the hourly event summary data.
pub async fn hourly(conn: &DbConn) -> QueryResult<Vec<HourlyEventSummary>> {
    let start_time = Utc::now().naive_utc() - Duration::days(1);

    let sql = "
        SELECT strftime('%Y-%m-%d %H:00:00', created_at) AS hour, COUNT(*) AS count
        FROM event
        WHERE created_at > ?
        GROUP BY strftime('%Y-%m-%d %H', created_at)
        ORDER BY hour;
    ";

    match conn
        .run(move |c| {
            sql_query(sql)
                .bind::<Timestamp, _>(start_time)
                .load::<HourlyEventSummary>(c)
        })
        .await
    {
        Ok(query) => Ok(query),
        Err(e) => {
            eprintln!("Failed to load hourly event summary: {e}");
            Err(Error::NotFound)
        }
    }
}

#[derive(Serialize, Deserialize, QueryableByName)]
pub struct UrlEventCount {
    #[diesel(sql_type = Text)]
    pub url: String,
    #[diesel(sql_type = BigInt)]
    pub count: i64,
}

/// # `urls`
/// Retrieves the top 25 most visited URLs from the past 7 days, ordered by visit count.
///
/// ## Arguments
/// * `conn` - A database connection.
///
/// ## Errors
/// * `Error::NotFound` - If the query fails.
///
/// ## Returns
/// `QueryResult<Vec<UrlEventCount>>` containing the top 25 most visited URLs.
pub async fn urls(conn: &DbConn) -> QueryResult<Vec<UrlEventCount>> {
    let start_time = Utc::now().naive_utc() - Duration::days(7);

    let sql = "
        SELECT url, COUNT(*) AS count
        FROM event
        WHERE event.created_at > ?
        GROUP BY url
        ORDER BY count DESC
        LIMIT 25;
    ";

    match conn
        .run(move |c| {
            sql_query(sql)
                .bind::<Timestamp, _>(start_time)
                .load::<UrlEventCount>(c)
        })
        .await
    {
        Ok(query) => Ok(query),
        Err(e) => {
            eprintln!("Failed to load top URLs: {e}");
            Err(Error::NotFound)
        }
    }
}

#[derive(Serialize, Deserialize, QueryableByName)]
pub struct BrowserVisitCount {
    #[diesel(sql_type = Text)]
    pub browser: String,
    #[diesel(sql_type = BigInt)]
    pub count: i64,
}

/// # `browsers`
/// Retrieves statistics on the top 25 browsers used by visitors over the past 7 days.
///
/// ## Arguments
/// * `conn` - A database connection.
///
/// ## Errors
/// * `Error::NotFound` - If the query fails.
///
/// ## Returns
/// `QueryResult<Vec<BrowserVisitCount>>` containing the top 25 most used browsers.
pub async fn browsers(conn: &DbConn) -> QueryResult<Vec<BrowserVisitCount>> {
    let start_time = Utc::now().naive_utc() - Duration::days(7);

    let sql = "
        SELECT browser, COUNT(*) AS count
        FROM collector
        WHERE collector.created_at > ?
          AND browser IS NOT NULL
        GROUP BY browser
        ORDER BY count DESC
        LIMIT 25;
    ";

    match conn
        .run(move |c| {
            sql_query(sql)
                .bind::<Timestamp, _>(start_time)
                .load::<BrowserVisitCount>(c)
        })
        .await
    {
        Ok(query) => Ok(query),
        Err(e) => {
            eprintln!("Failed to load top browsers: {e}");
            Err(Error::NotFound)
        }
    }
}

#[derive(Serialize, Deserialize, QueryableByName)]
pub struct OsBrowserVisitCount {
    #[diesel(sql_type = Text)]
    os: String,
    #[diesel(sql_type = Text)]
    browser: String,
    #[diesel(sql_type = BigInt)]
    count: i64,
}

/// # `os_browsers`
/// Retrieves statistics on the top 25 OS and browser combinations used by visitors over the past 7 days.
///
/// ## Arguments
/// * `conn` - A database connection.
///
/// ## Errors
/// * `Error::NotFound` - If the query fails.
///
/// ## Returns
/// `QueryResult<Vec<OsBrowserVisitCount>>` containing the top 25 most used browsers.
pub async fn os_browsers(conn: &DbConn) -> QueryResult<Vec<OsBrowserVisitCount>> {
    let start_time = Utc::now().naive_utc() - Duration::days(7);

    let sql = "
        SELECT os, browser, COUNT(*) AS count
        FROM collector
        WHERE collector.created_at > ?
          AND os IS NOT NULL
          AND browser IS NOT NULL
        GROUP BY os, browser
        ORDER BY count DESC
        LIMIT 25;
    ";

    match conn
        .run(move |c| {
            sql_query(sql)
                .bind::<Timestamp, _>(start_time)
                .load::<OsBrowserVisitCount>(c)
        })
        .await
    {
        Ok(query) => Ok(query),
        Err(e) => {
            eprintln!("Failed to load top os browsers: {e}");
            Err(Error::NotFound)
        }
    }
}

#[derive(Serialize, Deserialize, QueryableByName)]
pub struct ReferrerCount {
    #[diesel(sql_type = Text)]
    domain: String,
    #[diesel(sql_type = BigInt)]
    count: i64,
}

/// `referrers`
/// Retrieves statistics on the top 25 referring domains that brought visitors to the site over the past 7 days.
///
/// ## Arguments
/// * `conn` - A database connection.
///
/// ## Errors
/// * `Error::NotFound` - If the query fails.
///
/// ## Returns
/// `QueryResult<Vec<ReferrerCount>>` containing the top 25 most used browsers.
pub async fn referrers(conn: &DbConn) -> QueryResult<Vec<ReferrerCount>> {
    let start_time = Utc::now().naive_utc() - Duration::days(7);

    let sql = "
        SELECT
            CASE
                WHEN referrer IS NULL OR referrer = '' THEN 'direct'
                ELSE COALESCE(NULLIF(SUBSTR(referrer, INSTR(referrer, '//') + 2), ''), referrer)
                END AS domain,
            COUNT(*) AS count
        FROM event
        WHERE event.created_at > ?
        GROUP BY domain
        ORDER BY count DESC
        LIMIT 25;
    ";

    match conn
        .run(move |c| {
            sql_query(sql)
                .bind::<Timestamp, _>(start_time)
                .load::<ReferrerCount>(c)
        })
        .await
    {
        Ok(query) => Ok(query),
        Err(e) => {
            eprintln!("Failed to load top referrers: {e}");
            Err(Error::NotFound)
        }
    }
}

#[derive(QueryableByName, Serialize, Deserialize, Debug)]
pub struct HourlyEventCounts {
    #[diesel(sql_type = Integer)]
    pub day: i32, // 0 is Sunday and 6 is Saturday
    #[diesel(sql_type = Integer)]
    pub hour: i32, // Hour of the day (0-23)
    #[diesel(sql_type = BigInt)]
    pub count: i64, // The count of events in that hour
}

/// # `weekly`
/// Retrieves event counts grouped by day of week and hour of day for the past 7 days, enabling time-based traffic pattern analysis.
///
/// ## Arguments
/// * `conn` - A database connection.
///
/// ## Errors
/// * `Error::NotFound` - If the query fails.
///
/// ## Returns
/// `QueryResult<Vec<HourlyEventCounts>>` containing event counts grouped by day of week and hour of day.
pub async fn weekly(conn: &DbConn) -> QueryResult<Vec<HourlyEventCounts>> {
    let sql = "\
SELECT CAST(STRFTIME('%w', created_at) AS INTEGER) AS day,
        CAST(STRFTIME('%H', created_at) AS INTEGER) AS hour,
        COUNT(*)                                    AS count
FROM event
WHERE created_at >= DATETIME('now', '-7 days')
GROUP BY day, hour;";

    println!("Executing SQL query: {}", sql);

    match conn
        .run(move |c| sql_query(sql).load::<HourlyEventCounts>(c))
        .await
    {
        Ok(query) => Ok(query),
        Err(_) => Err(Error::NotFound),
    }
}

#[derive(QueryableByName, Debug)]
struct TrafficChange {
    #[diesel(sql_type = BigInt)]
    current_count: i64,
    #[diesel(sql_type = BigInt)]
    previous_count: i64,
}

/// # `percentages`
/// Calculates percentage changes in traffic volume between current and previous time periods (day, week, month) to show growth or decline trends.
///
/// ## Arguments
/// * `conn` - A database connection.
///
/// ## Errors
/// * `Error::NotFound` - If the query fails.
///
/// ## Returns
/// `QueryResult<serde_json::Value>` containing percentage changes in traffic volume.
pub async fn percentages(conn: &DbConn) -> QueryResult<serde_json::Value> {
    // Suppress Clippy warning about casting i64 to f64, which is acceptable
    // in this context because we're calculating percentages for website traffic,
    // where the values are unlikely to be so large that precision loss is significant.
    #[allow(clippy::cast_precision_loss)]
    let calc_percentage_change = |current: i64, previous: i64| -> f64 {
        if previous == 0 {
            if current == 0 { 0.0 } else { f64::INFINITY }
        } else {
            ((current as f64 - previous as f64) / previous as f64) * 100.0
        }
    };

    let intervals = vec![
        ("day", "-1 day", "-2 days"),
        ("week", "-7 days", "-14 days"),
        ("month", "-1 month", "-2 months"),
    ];

    let mut changes = serde_json::Map::new();

    for (label, current_interval, previous_interval) in intervals {
        let query = format!(
            "SELECT \
                (SELECT COUNT(*) FROM event WHERE created_at >= datetime('now', '{current_interval}')) AS current_count, \
                (SELECT COUNT(*) FROM event WHERE created_at BETWEEN datetime('now', '{previous_interval}') AND datetime('now', '{current_interval}')) AS previous_count",
        );

        let result = conn
            .run(move |c| sql_query(query).load::<TrafficChange>(c))
            .await?;

        let change = if let Some(tc) = result.first() {
            calc_percentage_change(tc.current_count, tc.previous_count)
        } else {
            0.0
        };

        changes.insert(label.to_string(), json!(change));
    }

    Ok(serde_json::Value::Object(changes))
}
