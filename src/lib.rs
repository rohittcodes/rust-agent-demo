use serde::Deserialize;

pub const WEATHER_API: &str = "https://api.open-meteo.com/v1/forecast";
pub const NEWS_API: &str = "https://newsapi.org/v2/top-headlines";
pub const NEWS_API_KEY: &str = "your_newsapi_key_here"; // Replace with actual API key
pub const GEOCODING_API: &str = "https://geocoding-api.open-meteo.com/v1/search";

#[derive(Debug, Deserialize)]
pub struct GeocodingResult {
    pub results: Option<Vec<GeoLocation>>,
}

#[derive(Debug, Deserialize)]
pub struct GeoLocation {
    pub latitude: f64,
    pub longitude: f64,
    pub name: String,
    pub country: Option<String>,
    pub admin1: Option<String>,
}

#[derive(Debug, Clone)]
pub struct LocationInfo {
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub country: Option<String>,
}

pub mod geocoding;
pub mod weather;
pub mod news;
pub mod alerts;
pub mod location;
pub mod llm; 