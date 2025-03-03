pub mod api_response;
pub mod models;
pub mod schema;

pub use api_response::ApiResponse;

/// # Database Connection Pool
/// Provides a SQLite connection pool using rocket_sync_db_pools.
/// This struct is used throughout the application to interact with the database.
#[rocket_sync_db_pools::database("sqlite_database")]
pub struct DbConn(diesel::SqliteConnection);
