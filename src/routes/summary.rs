use rocket::{get, serde::json::Json};
use serde_json::{Value, json};

use crate::{
    DbConn,
    api_response::ApiResponse,
    models::{
        browsers, events, five_minutes, hourly, os_browsers, percentages, referrers, urls, weekly,
    },
};

/// # `summary_get_five_minutes`
/// Retrieves the five-minute event summary for a given city.
///
/// ## Arguments
/// * `conn` - The database connection.
///
/// ## Returns
/// * `Json<Value>` - The five-minute event summary.
#[get("/five_minutes")]
pub async fn summary_get_five_minutes(conn: DbConn) -> Json<Value> {
    match five_minutes(&conn).await {
        Ok(summary) => ApiResponse::success(json!({
            "summary": summary
        })),
        Err(err) => ApiResponse::internal_error(&format!("Failed to retrieve map data: {err}")),
    }
}

/// # `summary_get_events`
/// Retrieves the event summary for a given city.
///
/// ## Arguments
/// * `conn` - The database connection.
///
/// ## Returns
/// * `Json<Value>` - The event summary.
#[get("/events")]
pub async fn summary_get_events(conn: DbConn) -> Json<Value> {
    match events(&conn).await {
        Ok(summary) => ApiResponse::success(json!({
            "summary": summary
        })),
        Err(err) => ApiResponse::internal_error(&format!("Failed to retrieve map data: {err}")),
    }
}

/// # `summary_get_hourly`
/// Retrieves the hourly event summary for a given city.
///
/// ## Arguments
/// * `conn` - The database connection.
///
/// ## Returns
/// * `Json<Value>` - The hourly event summary.
#[get("/hourly")]
pub async fn summary_get_hourly(conn: DbConn) -> Json<Value> {
    match hourly(&conn).await {
        Ok(summary) => ApiResponse::success(json!({
            "summary": summary
        })),
        Err(err) => ApiResponse::internal_error(&format!("Failed to retrieve map data: {err}")),
    }
}

/// # `summary_get_browsers`
/// Retrieves the top 25 most used browsers.
///
/// ## Arguments
/// * `conn` - The database connection.
///
/// ## Returns
/// * `Json<Value>` - The top 25 most used browsers.
#[get("/browsers")]
pub async fn summary_get_browsers(conn: DbConn) -> Json<Value> {
    match browsers(&conn).await {
        Ok(summary) => ApiResponse::success(json!({
            "summary": summary
        })),
        Err(err) => ApiResponse::internal_error(&format!("Failed to retrieve map data: {err}")),
    }
}

/// # `summary_get_os_browsers`
/// Retrieves the top 25 most used operating systems and browsers.
///
/// ## Arguments
/// * `conn` - The database connection.
///
/// ## Returns
/// * `Json<Value>` - The top 25 most used operating systems and browsers.
#[get("/os_browsers")]
pub async fn summary_get_os_browsers(conn: DbConn) -> Json<Value> {
    match os_browsers(&conn).await {
        Ok(summary) => ApiResponse::success(json!({
            "summary": summary
        })),
        Err(err) => ApiResponse::internal_error(&format!("Failed to retrieve map data: {err}")),
    }
}

/// # `summary_get_referrers`
/// Retrieves the top 25 most used referrers.
///
/// ## Arguments
/// * `conn` - The database connection.
///
/// ## Returns
/// * `Json<Value>` - The top 25 most used referrers.
#[get("/referrers")]
pub async fn summary_get_referrers(conn: DbConn) -> Json<Value> {
    match referrers(&conn).await {
        Ok(summary) => ApiResponse::success(json!({
            "summary": summary
        })),
        Err(err) => ApiResponse::internal_error(&format!("Failed to retrieve map data: {err}")),
    }
}

/// # `summary_get_weekly_event_counts`
/// Retrieves the weekly event counts for the last 7 days.
///
/// ## Arguments
/// * `conn` - The database connection.
///
/// ## Returns
/// * `Json<Value>` - The weekly event counts for the last 7 days.
#[get("/weekly")]
pub async fn summary_get_weekly_event_counts(conn: DbConn) -> Json<Value> {
    match weekly(&conn).await {
        Ok(summary) => ApiResponse::success(json!({
            "summary": summary
        })),
        Err(err) => ApiResponse::internal_error(&format!("Failed to retrieve map data: {err}")),
    }
}

/// # `summary_get_percentages`
/// Calculates percentage changes in traffic volume between current and previous time periods (day, week, month) to show growth or decline trends.
///
/// ## Arguments
/// * `conn` - The database connection.
///
/// ## Returns
/// * `Json<Value>` - The percentage changes in traffic volume between current and previous time periods.
#[get("/percentages")]
pub async fn summary_get_percentages(conn: DbConn) -> Json<Value> {
    match percentages(&conn).await {
        Ok(summary) => ApiResponse::success(json!({
            "summary": summary
        })),
        Err(err) => ApiResponse::internal_error(&format!("Failed to retrieve map data: {err}")),
    }
}

/// # `summary_get_urls`
/// Retrieves the top 25 most visited URLs from the past 7 days, ordered by visit count.
///
/// ## Arguments * `conn` - The database connection.
///
/// ## Returns
/// * `Json<Value>` - The top 25 most visited URLs from the past 7 days, ordered by visit count.
#[get("/urls")]
pub async fn summary_get_urls(conn: DbConn) -> Json<Value> {
    match urls(&conn).await {
        Ok(summary) => ApiResponse::success(json!({
            "summary": summary
        })),
        Err(err) => ApiResponse::internal_error(&format!("Failed to retrieve map data: {err}")),
    }
}
