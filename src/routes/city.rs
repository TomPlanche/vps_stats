use crate::{
    ApiResponse, DbConn,
    models::{City, CityQuery},
    paginated::set_pagination_defaults,
};
use rocket::{get, post, serde::json::Json};
use serde_json::{Value, json};

/// # `city_insert`
/// Handles POST requests to insert a new city.
///
/// ## Arguments
/// * `city_data` - city data from request
/// * `conn` - Database connection
///
/// ## Panics
/// If the regex pattern is invalid.
#[post("/", format = "application/json", data = "<city_data>")]
pub async fn city_insert(city_data: Json<CityQuery>, conn: DbConn) -> Json<Value> {
    let city = city_data.into_inner().into();

    match City::find_or_create(city, &conn).await {
        Ok(id) => ApiResponse::created(json!({
            "message": &format!("City #{id} recorded successfully")
        })),
        Err(e) => ApiResponse::internal_error(&format!("Failed to record city: {e}")),
    }
}

/// # `city_all`
/// Handles GET requests to retrieve all cities.
///
/// ## Arguments
/// * `page` - Page number for pagination
/// * `per_page` - Number of items per page
/// * `conn` - Database connection
///
/// ## Returns
/// * `Json<Value>` - JSON response containing events data
#[get("/?<page>&<per_page>")]
pub async fn city_get(page: Option<i64>, per_page: Option<i64>, conn: DbConn) -> Json<Value> {
    let (page, per_page) = set_pagination_defaults(page, per_page);

    match City::all(page, per_page, &conn).await {
        Ok(cities) => ApiResponse::success(json!(cities)),
        Err(e) => ApiResponse::internal_error(&format!("Failed to retrieve cities: {e}")),
    }
}
