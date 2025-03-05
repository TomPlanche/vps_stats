use std::env;

use rocket::figment::Figment;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AppConfig {
    pub port: i16,
    pub address: String,
}

impl AppConfig {
    #[must_use]
    pub fn new() -> Self {
        dotenv::dotenv().ok();

        let address = env::var("APP_URL").unwrap_or("127.0.0.1:5775".to_string());
        let port: i16 = env::var("SERVICE_PORT")
            .unwrap_or("5775".to_string())
            .parse()
            .unwrap_or(5775);

        Self { port, address }
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl From<AppConfig> for Figment {
    fn from(config: AppConfig) -> Self {
        rocket::Config::figment()
            .merge(("port", config.port))
            .merge(("address", config.address))
    }
}
