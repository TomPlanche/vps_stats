use std::net::IpAddr;

use crate::paginated::Paginate;
use chrono::NaiveDateTime;
use diesel::{
    ExpressionMethods, OptionalExtension, QueryDsl, QueryResult, RunQueryDsl,
    dsl::sql,
    insert_into,
    prelude::{Identifiable, Insertable, Queryable},
    select,
    sql_types::Integer,
};
use reqwest;
use serde::{Deserialize, Serialize};

use crate::services::ip_location;
use crate::{DbConn, paginated::PaginationResult, schema::city};

#[derive(Deserialize, Identifiable, Insertable, Queryable, Serialize, Debug)]
#[diesel(table_name = city)]
#[serde(crate = "rocket::serde")]
pub struct City {
    pub id: Option<i32>,
    pub name: String,
    pub country: String,
    pub latitude: Option<f32>,
    pub longitude: Option<f32>,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct CityQuery {
    pub name: String,
    pub country: String,
}

impl From<CityQuery> for City {
    fn from(query: CityQuery) -> Self {
        City {
            id: None,
            name: query.name,
            country: query.country,
            latitude: None,
            longitude: None,
            created_at: None,
        }
    }
}

impl City {
    /// # `all`
    /// Returns all cities in the database.
    ///
    /// ## Arguments
    /// * `page` - The page number
    /// * `per_page` - The number of items per page
    /// * `conn` - The database connection.
    ///
    /// ## Errors
    /// * `Error::NotFound` - If the city is not found.
    ///
    /// ## Returns
    /// * `QueryResult<Vec<City>>` - The list of cities.
    pub async fn all(
        page: i64,
        per_page: i64,
        conn: &DbConn,
    ) -> QueryResult<PaginationResult<City>> {
        conn.run(move |c| {
            city::table
                .order(city::id)
                .paginate(page)
                .per_page(per_page)
                .load_and_count_pages(c)
        })
        .await
    }

    /// # `insert`
    /// Inserts a new city into the database and returns its ID.
    ///
    /// ## Arguments
    /// * `city` - The city to insert.
    /// * `ip` - The IP address of the client.
    /// * `conn` - The database connection.
    ///
    /// ## Errors
    /// * `Error::NotFound` - If the city is not found.
    ///
    /// ## Returns
    /// * `QueryResult<i32>` - The ID of the inserted city.
    pub async fn insert(mut city: City, ip: IpAddr, conn: &DbConn) -> QueryResult<i32> {
        // Try to get location data from IP
        if let Ok(ip_city) = Self::from_ip(&ip.to_string()).await {
            // Update coordinates if they're not already set
            if city.latitude.is_none() {
                city.latitude = ip_city.latitude;
            }
            if city.longitude.is_none() {
                city.longitude = ip_city.longitude;
            }
        }

        conn.run(move |c| {
            insert_into(city::table).values(&city).execute(c)?;

            // Get the ID of the last inserted row
            let id = select(sql::<Integer>("last_insert_rowid()")).first(c)?;

            Ok(id)
        })
        .await
    }

    /// # `find_by_name_and_country`
    /// Finds a city by name and country.
    ///
    /// ## Arguments
    /// * `name_param` - The name of the city.
    /// * `country_param` - The country of the city.
    /// * `conn` - The database connection.
    ///
    /// ## Errors
    /// * `Error::NotFound` - If the city is not found.
    ///
    /// ## Returns
    /// * `QueryResult<Option<City>>` - The found city, or None if not found.
    pub async fn find_by_name_and_country(
        name_param: String,
        country_param: String,
        conn: &DbConn,
    ) -> QueryResult<Option<City>> {
        let name_lower = name_param.to_lowercase();
        let country_lower = country_param.to_lowercase();

        conn.run(move |c| {
            city::dsl::city
                .filter(city::dsl::name.eq(name_lower))
                .filter(city::dsl::country.eq(country_lower))
                .first(c)
                .optional()
        })
        .await
    }

    /// # `from_ip`
    /// Creates a new City instance from an IP address by looking up its location
    ///
    /// ## Arguments
    /// * `ip` - The IP address to look up
    ///
    /// ## Errors
    /// * If the IP location lookup fails.
    ///
    /// ## Returns
    /// * `Result<City, reqwest::Error>` - The city info or an error
    async fn from_ip(ip: &str) -> Result<Self, reqwest::Error> {
        let info = ip_location::get_city_info(ip).await?;
        let (latitude, longitude) = info.coordinates().unwrap_or((0.0, 0.0));

        Ok(City {
            id: None,
            name: info.city,
            country: info.country,
            latitude: Some(latitude),
            longitude: Some(longitude),
            created_at: None,
        })
    }
}
