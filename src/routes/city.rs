use crate::api_response::ApiResponse;
use crate::{
    DbConn,
    models::{City, CityQuery},
    paginated::set_pagination_defaults,
};
use rocket::{get, post, serde::json::Json};
use serde_json::{Value, json};
use std::net::IpAddr;

/// # `city_insert`
/// Handles POST requests to insert a new city.
///
/// ## Arguments
/// * `city_data` - city data from request
/// * `ip` - IP address of the client
/// * `conn` - Database connection
///
/// ## Panics
/// If the regex pattern is invalid.
#[post("/", format = "application/json", data = "<city_data>")]
pub async fn city_insert(city_data: Json<CityQuery>, ip: IpAddr, conn: DbConn) -> Json<Value> {
    let mut city: City = city_data.into_inner().into();
    city.name = city.name.to_lowercase();
    city.country = city.country.to_lowercase();

    let existing_city =
        City::find_by_name_and_country(city.name.clone(), city.country.clone(), &conn).await;

    let new_id = match existing_city.unwrap() {
        Some(found) => Ok(found.id.unwrap_or_default()),
        None => City::insert(city, ip, &conn).await,
    };

    match new_id {
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
