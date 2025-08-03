use anyhow::{Result, Context};
use crate::{LocationInfo, WEATHER_API};

pub struct Weather {
    location_info: LocationInfo,
}

impl Weather {
    pub fn new(location_info: LocationInfo) -> Self {
        Self { location_info }
    }

    pub async fn get_weather(&self) -> Result<String> {
        let url = format!(
            "{base}?latitude={lat}&longitude={lon}&current_weather=true&hourly=temperature_2m,relative_humidity_2m,wind_speed_10m&daily=temperature_2m_max,temperature_2m_min,precipitation_sum&timezone=auto&forecast_days=3",
            base = WEATHER_API, 
            lat = self.location_info.latitude, 
            lon = self.location_info.longitude
        );
        
        let client = reqwest::Client::new();
        let response = client
            .get(&url)
            .header("User-Agent", "multi-tool-agent/1.0")
            .send()
            .await
            .context("Failed to fetch weather data")?;
        
        let weather_data = response
            .text()
            .await
            .context("Failed to read weather response")?;
        
        Ok(format!("Current weather and 3-day forecast for {} ({}°N, {}°E): {}", 
                  self.location_info.name, 
                  self.location_info.latitude, 
                  self.location_info.longitude,
                  weather_data))
    }
} 