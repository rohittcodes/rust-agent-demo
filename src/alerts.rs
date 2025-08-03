use anyhow::{Result, Context};
use crate::{LocationInfo, WEATHER_API};

pub struct Alerts {
    location_info: LocationInfo,
}

impl Alerts {
    pub fn new(location_info: LocationInfo) -> Self {
        Self { location_info }
    }

    pub async fn get_alerts(&self) -> Result<String> {
        let url = format!(
            "{base}?latitude={lat}&longitude={lon}&current_weather=true&daily=temperature_2m_max,temperature_2m_min,precipitation_sum,wind_speed_10m_max&alerts=true&timezone=auto",
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
            .context("Failed to fetch alerts data")?;
        
        let alerts_data = response
            .text()
            .await
            .context("Failed to read alerts response")?;
        
        Ok(format!("Weather alerts and warnings for {}: {}", self.location_info.name, alerts_data))
    }
} 