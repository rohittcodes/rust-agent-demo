use anyhow::{Result, Context};
use crate::{GeocodingResult, LocationInfo, GEOCODING_API};

pub struct GeocodingService;

impl GeocodingService {
    pub async fn get_coordinates(location: &str, country_code: Option<&str>) -> Result<LocationInfo> {
        let mut url = format!("{base}?name={location}&count=1&language=en&format=json", 
                             base = GEOCODING_API, location = urlencoding::encode(location));
        
        if let Some(country) = country_code {
            url.push_str(&format!("&country={}", country));
        }

        let client = reqwest::Client::new();
        let response = client
            .get(&url)
            .header("User-Agent", "multi-tool-agent/1.0")
            .send()
            .await
            .context("Failed to send geocoding request")?;
        
        let geocoding_result: GeocodingResult = response
            .json()
            .await
            .context("Failed to parse geocoding response")?;
        
        match geocoding_result.results.and_then(|mut results| results.pop()) {
            Some(geo_location) => Ok(LocationInfo {
                name: geo_location.name,
                latitude: geo_location.latitude,
                longitude: geo_location.longitude,
                country: geo_location.country,
            }),
            None => Err(anyhow::anyhow!("Location '{}' not found", location)),
        }
    }
} 