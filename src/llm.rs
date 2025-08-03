use anyhow::{Result, Context};
use crate::{LocationInfo, weather::Weather, news::News, alerts::Alerts, location::LocationDetails};
use std::env;

pub struct LLMProcessor {
    weather: Weather,
    news: News,
    alerts: Alerts,
    location_details: LocationDetails,
    location_info: LocationInfo,
}

impl LLMProcessor {
    pub fn new(location_info: LocationInfo) -> Self {
        Self {
            weather: Weather::new(location_info.clone()),
            news: News::new(location_info.clone()),
            alerts: Alerts::new(location_info.clone()),
            location_details: LocationDetails::new(location_info.clone()),
            location_info,
        }
    }

    pub async fn process_query(&self, query: &str, use_gemini: bool, use_claude: bool) -> Result<String> {
        if use_gemini {
            self.process_with_gemini(query).await
        } else if use_claude {
            self.process_with_claude(query).await
        } else {
            Err(anyhow::anyhow!("No LLM API selected. Use --gemini or --claude flag."))
        }
    }

    async fn process_with_gemini(&self, query: &str) -> Result<String> {
        let api_key = env::var("GEMINI_API_KEY")
            .context("GEMINI_API_KEY environment variable not set")?;
        
        let client = reqwest::Client::new();
        
        let system_prompt = format!(
            "You are a location information agent for {}. You have access to these tools:\n\
            - get_weather(): Get current weather and 3-day forecast\n\
            - get_news(): Get latest news headlines\n\
            - get_alerts(): Get weather alerts and warnings\n\
            - get_location_info(): Get location details\n\n\
            Based on the user's query, determine which tool to call and provide a helpful response. \
            Be specific about what action you're taking and respond in a conversational manner.",
            self.location_info.name
        );

        let full_prompt = format!("{}\n\nUser query: {}\n\nResponse:", system_prompt, query);

        let url = format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash-latest:generateContent?key={}", api_key);
        
        println!("DEBUG: Calling Gemini API with URL: {}", url);
        println!("DEBUG: Prompt: {}", &full_prompt[..full_prompt.len().min(200)]);

        let response = client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({
                "contents": [{
                    "parts": [{
                        "text": full_prompt
                    }]
                }],
                "generationConfig": {
                    "temperature": 0.7,
                    "topK": 40,
                    "topP": 0.95,
                    "maxOutputTokens": 500
                }
            }))
            .send()
            .await
            .context("Failed to call Gemini API")?;
            
        println!("DEBUG: Gemini response status: {}", response.status());

        let response_data: serde_json::Value = response.json().await
            .context("Failed to parse Gemini response")?;

        let content = response_data["candidates"][0]["content"]["parts"][0]["text"]
            .as_str()
            .context("Invalid Gemini response format")?;

        self.execute_tool_based_on_response(content, query).await
    }

    async fn process_with_claude(&self, query: &str) -> Result<String> {
        let api_key = env::var("ANTHROPIC_API_KEY")
            .context("ANTHROPIC_API_KEY environment variable not set")?;
        
        let client = reqwest::Client::new();
        
        let system_prompt = format!(
            "You are a location information agent for {}. You have access to these tools:\n\
            - get_weather(): Get current weather and 3-day forecast\n\
            - get_news(): Get latest news headlines\n\
            - get_alerts(): Get weather alerts and warnings\n\
            - get_location_info(): Get location details\n\n\
            Based on the user's query, determine which tool to call and provide a helpful response. \
            Be specific about what action you're taking and respond in a conversational manner.",
            self.location_info.name
        );

        let response = client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", api_key)
            .header("Content-Type", "application/json")
            .header("anthropic-version", "2023-06-01")
            .json(&serde_json::json!({
                "model": "claude-3-haiku-20240307",
                "max_tokens": 500,
                "system": system_prompt,
                "messages": [{
                    "role": "user",
                    "content": query
                }]
            }))
            .send()
            .await
            .context("Failed to call Claude API")?;

        let response_data: serde_json::Value = response.json().await
            .context("Failed to parse Claude response")?;

        let content = response_data["content"][0]["text"]
            .as_str()
            .context("Invalid Claude response format")?;

        self.execute_tool_based_on_response(content, query).await
    }

    async fn execute_tool_based_on_response(&self, llm_response: &str, original_query: &str) -> Result<String> {
        let response_lower = llm_response.to_lowercase();
        let query_lower = original_query.to_lowercase();
        
        println!("DEBUG: execute_tool_based_on_response called");
        println!("DEBUG: Original query: '{}'", original_query);
        println!("DEBUG: LLM response: '{}'", llm_response);
        
        // More robust tool selection based primarily on the original query
        let tool_result = if query_lower.contains("alert") || query_lower.contains("warning") || 
                              query_lower.contains("emergency") || query_lower.contains("danger") {
            println!("DEBUG: Calling alerts tool");
            Some(self.alerts.get_alerts().await?)
        } else if query_lower.contains("news") || query_lower.contains("headlines") || 
                   query_lower.contains("latest") || query_lower.contains("breaking") {
            println!("DEBUG: Calling news tool");
            Some(self.news.get_news().await?)
        } else if query_lower.contains("weather") || query_lower.contains("temperature") || 
                   query_lower.contains("forecast") || query_lower.contains("climate") {
            println!("DEBUG: Calling weather tool");
            Some(self.weather.get_weather().await?)
        } else if query_lower.contains("location") || query_lower.contains("coordinates") || 
                   query_lower.contains("where") || query_lower.contains("info") {
            println!("DEBUG: Calling location tool");
            Some(self.location_details.get_location_info().await?)
        } else {
            println!("DEBUG: No direct match, checking LLM response for tool hints");
            // Fallback: check if LLM response mentions any tools
            if response_lower.contains("weather") || response_lower.contains("get_weather") {
                println!("DEBUG: LLM mentioned weather, calling weather tool");
                Some(self.weather.get_weather().await?)
            } else if response_lower.contains("news") || response_lower.contains("get_news") {
                println!("DEBUG: LLM mentioned news, calling news tool");
                Some(self.news.get_news().await?)
            } else if response_lower.contains("alert") || response_lower.contains("get_alerts") {
                println!("DEBUG: LLM mentioned alerts, calling alerts tool");
                Some(self.alerts.get_alerts().await?)
            } else if response_lower.contains("location") || response_lower.contains("get_location") {
                println!("DEBUG: LLM mentioned location, calling location tool");
                Some(self.location_details.get_location_info().await?)
            } else {
                println!("DEBUG: No tool called - returning LLM response only");
                None
            }
        };

        match tool_result {
            Some(data) => Ok(format!("{}\n\n{}", llm_response, data)),
            None => Ok(llm_response.to_string()),
        }
    }
} 