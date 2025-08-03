use anyhow::Result;
use crate::LocationInfo;

pub struct LocationDetails {
    location_info: LocationInfo,
}

impl LocationDetails {
    pub fn new(location_info: LocationInfo) -> Self {
        Self { location_info }
    }

    pub async fn get_location_info(&self) -> Result<String> {
        Ok(format!(
            "Location Details:\n- Name: {}\n- Coordinates: {:.4}°N, {:.4}°E\n- Country: {}\n- Ready to provide weather, news, and alerts for this location.",
            self.location_info.name,
            self.location_info.latitude,
            self.location_info.longitude,
            self.location_info.country.as_deref().unwrap_or("Unknown")
        ))
    }
} 