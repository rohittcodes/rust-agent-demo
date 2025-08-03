use anyhow::{Result, Context};
use clap::Parser;
use tracing_subscriber;
use std::io::Write;
use std::env;

use rust_agent_demo::{geocoding::GeocodingService, llm::LLMProcessor};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    location: String,
    
    #[arg(short, long)]
    country: Option<String>,
    
    #[arg(long)]
    gemini: bool,
    
    #[arg(long)]
    claude: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().init();

    let args = Args::parse();
    
    if args.gemini && env::var("GEMINI_API_KEY").is_err() {
        eprintln!("GEMINI_API_KEY environment variable not set");
        eprintln!("Set it with: export GEMINI_API_KEY='your-key-here'");
        eprintln!("Get your key from: https://makersuite.google.com/app/apikey");
        std::process::exit(1);
    }
    
    if args.claude && env::var("ANTHROPIC_API_KEY").is_err() {
        eprintln!("ANTHROPIC_API_KEY environment variable not set");
        eprintln!("Set it with: export ANTHROPIC_API_KEY='your-key-here'");
        std::process::exit(1);
    }

    if !args.gemini && !args.claude {
        eprintln!("Please specify either --gemini or --claude flag");
        eprintln!("Example: cargo run -- --location 'New York' --gemini");
        std::process::exit(1);
    }
    
    eprintln!("Looking up coordinates for '{}'...", args.location);
    let location_info = GeocodingService::get_coordinates(
        &args.location, 
        args.country.as_deref()
    ).await.context("Failed to get location coordinates")?;
    
    eprintln!("Found location: {} at coordinates ({:.4}, {:.4})", 
             location_info.name, location_info.latitude, location_info.longitude);
    
    if let Some(country) = &location_info.country {
        eprintln!("Country: {}", country);
    }

    let processor = LLMProcessor::new(location_info.clone());
    
    let llm_type = if args.gemini { "Gemini" } else { "Claude" };
    println!("\nLocation Agent for: {} (using {})", location_info.name, llm_type);
    println!("Ask me anything about this location!");
    println!("Type 'quit' to exit.\n");
    
    loop {
        print!("You: ");
        std::io::stdout().flush().unwrap();
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        if input.to_lowercase() == "quit" {
            println!("Goodbye!");
            break;
        }
        
        if input.is_empty() {
            continue;
        }
        
        match processor.process_query(input, args.gemini, args.claude).await {
            Ok(response) => println!("\nAgent: {}\n", response),
            Err(e) => println!("\nAgent: Error: {}\n", e),
        }
    }
    
    Ok(())
}