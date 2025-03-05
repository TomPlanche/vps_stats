use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
pub struct IpInfoResponse {
    pub ip: String,
    pub city: String,
    pub country: String,
    pub loc: String,
    pub org: Option<String>,
    pub postal: Option<String>,
    pub timezone: Option<String>,
}

impl IpInfoResponse {
    /// # `coordinates`
    /// Get the geographical coordinates (latitude, longitude) of the IP address.
    ///
    /// ## Errors
    /// Returns an error if the coordinates cannot be parsed as floating-point numbers.
    ///
    /// ## Returns
    /// * `Some((latitude, longitude))` if the coordinates are available, otherwise `None`.
    #[must_use]
    pub fn coordinates(&self) -> Option<(f32, f32)> {
        let coords: Vec<&str> = self.loc.split(',').collect();
        if coords.len() == 2 {
            let lat = coords[0].parse::<f32>().ok()?;
            let lon = coords[1].parse::<f32>().ok()?;
            Some((lat, lon))
        } else {
            None
        }
    }
}

/// # `get_city_info`
/// Get city information for a given IP address.
///
/// This function sends a request to the `IPInfo` API to retrieve information about the city associated with the provided IP address.
/// It requires an API token which should be set as the environment variable `IPINFO_TOKEN`.
///
/// ## Arguments
/// * `ip` - The IP address for which to retrieve city information.
///
/// ## Errors
/// Returns an error if the request to the `IPInfo` API fails.
///
/// ## Returns
/// * `IpInfoResponse` result
pub async fn get_city_info(ip: &str) -> Result<IpInfoResponse, reqwest::Error> {
    let token = env::var("IPINFO_TOKEN").unwrap_or_default();

    let base_url = if ip.contains(':') {
        "https://v6.ipinfo.io"
    } else {
        "https://ipinfo.io"
    };

    let url = format!("{base_url}/{ip}");
    let client = reqwest::Client::new();

    client
        .get(url)
        .header("Authorization", format!("Bearer {token}"))
        .send()
        .await?
        .json::<IpInfoResponse>()
        .await
}
