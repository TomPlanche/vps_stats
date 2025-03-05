use rocket_sync_db_pools::database;

pub mod api_response;
pub mod config;
pub mod cors;
pub mod models;
pub mod paginated;
pub mod routes;
pub mod schema;
pub mod services;

/// # Database Connection Pool
/// Provides a SQLite connection pool using rocket_sync_db_pools.
/// This struct is used throughout the application to interact with the database.
#[database("sqlite_database")]
pub struct DbConn(diesel::SqliteConnection);

// CONSTS
pub const MAX_PER_PAGE: i64 = 100; // Prevent excessive page sizes
pub const DEFAULT_PER_PAGE: i64 = 10;
pub const DEFAULT_PAGE: i64 = 1;
