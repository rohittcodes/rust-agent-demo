use anyhow::{Result, Context};
use crate::{LocationInfo, NEWS_API, NEWS_API_KEY};
use std::env;

pub struct News {
    location_info: LocationInfo,
}

impl News {
    pub fn new(location_info: LocationInfo) -> Self {
        Self { location_info }
    }

    pub async fn get_news(&self) -> Result<String> {
        let api_key = env::var("NEWS_API_KEY").unwrap_or_else(|_| NEWS_API_KEY.to_string());
        
        if api_key == "your_newsapi_key_here" {
            return Ok(format!("News service requires a valid API key. Please set NEWS_API_KEY environment variable or update the constant with your NewsAPI.org key to get news for {}", self.location_info.name));
        }
        
        let search_query = format!("{} {}", 
            self.location_info.name,
            self.location_info.country.as_deref().unwrap_or("")
        );
        
        let url = format!(
            "{base}?q={query}&apiKey={api_key}&pageSize=10&sortBy=publishedAt&language=en",
            base = NEWS_API, 
            query = urlencoding::encode(&search_query),
            api_key = api_key
        );
        
        let client = reqwest::Client::new();
        let response = client
            .get(&url)
            .header("User-Agent", "multi-tool-agent/1.0")
            .send()
            .await
            .context("Failed to fetch news data")?;
        
        let news_data = response
            .text()
            .await
            .context("Failed to read news response")?;
        
        Ok(format!("Latest news headlines for {}: {}", self.location_info.name, news_data))
    }
} 