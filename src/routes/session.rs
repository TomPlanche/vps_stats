use rocket::{get, serde::json::Json};
use serde_json::json;

use crate::{
    DbConn,
    api_response::ApiResponse,
    models::{map, retrieve_sessions},
};

/// # `get_sessions`
/// Handle the request to retrieve the last 30 recent visitor sessions.
///
/// ## Arguments
/// * `conn` - The database connection.
///
/// ## Returns
/// * `Json<serde_json::Value>` - The JSON response containing the sessions.
#[get("/")]
pub async fn session_get_sessions(conn: DbConn) -> Json<serde_json::Value> {
    match retrieve_sessions(&conn).await {
        Ok(sessions) => ApiResponse::success(json!({
            "sessions": sessions
        })),
        Err(err) => ApiResponse::internal_error(&format!("Failed to retrieve sessions: {err}")),
    }
}

/// # `get_map_data`
/// Handle the request to retrieve the map data.
///
/// ## Arguments
/// * `conn` - The database connection.
///
/// ## Returns
/// * `Json<serde_json::Value>` - The JSON response containing the map data.
#[get("/map")]
pub async fn session_get_map_data(conn: DbConn) -> Json<serde_json::Value> {
    match map(&conn).await {
        Ok(data) => ApiResponse::success(json!({
            "cities": data
        })),
        Err(err) => ApiResponse::internal_error(&format!("Failed to retrieve map data: {err}")),
    }
}
