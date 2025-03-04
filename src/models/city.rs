use crate::paginated::Paginate;
use chrono::NaiveDateTime;
use diesel::{
    ExpressionMethods, OptionalExtension, QueryDsl, QueryResult, RunQueryDsl,
    prelude::{Identifiable, Insertable, Queryable},
};
use serde::{Deserialize, Serialize};

use crate::{DbConn, paginated::PaginationResult, schema::city};

#[derive(Deserialize, Identifiable, Insertable, Queryable, Serialize)]
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
    /// # `find_or_create`
    /// Finds a city by name and country, or creates it if it doesn't exist.
    ///
    /// ## Arguments
    /// * `city` - The city to find or create.
    /// * `conn` - The database connection.
    ///
    /// ## Returns
    /// * `QueryResult<i32>` - The ID of the found or created city.
    pub async fn find_or_create(city: City, conn: &DbConn) -> QueryResult<i32> {
        let existing_city = Self::find_by_name_and_country(
            city.name.clone(), // Clone here since we might need original case for insert
            city.country.clone(),
            conn,
        )
        .await?;

        match existing_city {
            Some(found) => Ok(found.id.unwrap_or_default()),
            None => Self::insert(city, conn).await,
        }
    }

    /// # `all`
    /// Returns all cities in the database.
    ///
    /// ## Arguments
    /// * `page` - The page number
    /// * `per_page` - The number of items per page
    /// * `conn` - The database connection.
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
    /// * `conn` - The database connection.
    ///
    /// ## Returns
    /// * `QueryResult<i32>` - The ID of the inserted city.
    async fn insert(city: City, conn: &DbConn) -> QueryResult<i32> {
        conn.run(move |c| {
            diesel::insert_into(city::table).values(&city).execute(c)?;

            // Get the ID of the last inserted row
            let id = diesel::select(diesel::dsl::sql::<diesel::sql_types::Integer>(
                "last_insert_rowid()",
            ))
            .first(c)?;

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
    /// ## Returns
    /// * `QueryResult<Option<City>>` - The found city, or None if not found.
    async fn find_by_name_and_country(
        name_param: String,
        country_param: String,
        conn: &DbConn,
    ) -> QueryResult<Option<City>> {
        use crate::schema::city::dsl::*;

        let name_lower = name_param.to_lowercase();
        let country_lower = country_param.to_lowercase();

        conn.run(move |c| {
            city.filter(name.eq(name_lower))
                .filter(country.eq(country_lower))
                .first(c)
                .optional()
        })
        .await
    }
}
