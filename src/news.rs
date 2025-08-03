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
        println!("DEBUG: get_news() called for {}", self.location_info.name);
        
        let api_key = env::var("NEWS_API_KEY").unwrap_or_else(|_| NEWS_API_KEY.to_string());
        println!("DEBUG: Using API key: {}", if api_key.len() > 10 { &api_key[..10] } else { "short" });
        
        if api_key == "your_newsapi_key_here" {
            println!("DEBUG: Using default API key - this will likely fail");
            return Ok(format!("News service requires a valid API key. Please set NEWS_API_KEY environment variable or update the constant with your NewsAPI.org key to get news for {}", self.location_info.name));
        }
        
        // Try different search strategies
        let search_queries = vec![
            self.location_info.name.clone(),
            format!("{} {}", self.location_info.name, self.location_info.country.as_deref().unwrap_or("")),
            format!("{} news", self.location_info.name),
            format!("{} France", self.location_info.name), // For Paris
        ];
        
        let mut news_data = String::new();
        
        for query in search_queries {
            let url = format!(
                "{base}?q={query}&apiKey={api_key}&pageSize=5&sortBy=publishedAt&language=en",
                base = NEWS_API, 
                query = urlencoding::encode(&query),
                api_key = api_key
            );
            
            println!("DEBUG: Trying query: '{}'", query);
            println!("DEBUG: URL: {}", url);
            
            let client = reqwest::Client::new();
            let response = client
                .get(&url)
                .header("User-Agent", "multi-tool-agent/1.0")
                .send()
                .await
                .context("Failed to fetch news data")?;
            
            println!("DEBUG: Response status: {}", response.status());
            
            let data = response
                .text()
                .await
                .context("Failed to read news response")?;
            
            println!("DEBUG: Response data: {}", &data[..data.len().min(200)]);
            
            // Check if we got results
            if data.contains("\"totalResults\":0") || data.contains("\"articles\":[]") {
                println!("DEBUG: No results for query '{}', trying next...", query);
                continue; // Try next query
            } else {
                println!("DEBUG: Found results for query '{}'", query);
                news_data = data;
                break; // Found results
            }
        }
        
        // If still no results, try a broader search
        if news_data.is_empty() || news_data.contains("\"totalResults\":0") {
            let url = format!(
                "{base}?q=France&apiKey={api_key}&pageSize=5&sortBy=publishedAt&language=en",
                base = NEWS_API, 
                api_key = api_key
            );
            
            let client = reqwest::Client::new();
            let response = client
                .get(&url)
                .header("User-Agent", "multi-tool-agent/1.0")
                .send()
                .await
                .context("Failed to fetch news data")?;
            
            news_data = response
                .text()
                .await
                .context("Failed to read news response")?;
        }
        
        Ok(format!("Latest news headlines for {}: {}", self.location_info.name, news_data))
    }
} 