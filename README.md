# Rust Agents: Location-Based AI Assistant

A modular Rust application that combines LLM APIs (Gemini/Claude) with real-time location data to provide weather, news, alerts, and location information through natural language queries.

## Features

- **Multi-LLM Support**: Works with Google Gemini and Anthropic Claude APIs
- **Real-time Data**: Weather, news, alerts, and location information
- **Modular Architecture**: Clean separation of tools and services
- **Interactive Chat**: Natural language interface
- **Location Intelligence**: Automatic geocoding and coordinate resolution

## Quick Start

### Prerequisites

1. **Get API Keys**:
   - [Gemini API Key](https://aistudio.google.com/apikey)
   - [Claude API Key](https://console.anthropic.com/)
   - [NewsAPI Key](https://newsapi.org/) (optional)

2. **Set Environment Variables**:
   ```bash
   export GEMINI_API_KEY="your-gemini-key"
   export ANTHROPIC_API_KEY="your-claude-key"
   export NEWS_API_KEY="your-newsapi-key"  # optional
   ```

### Installation

```bash
git clone https://github.com/rohittcodes/rust-agent-demo
cd rust-agent-demo
cargo build --release
```

### Usage

**With Gemini:**
```bash
cargo run -- --location "Tokyo" --gemini
```

**With Claude:**
```bash
cargo run -- --location "Paris" --claude
```

**Example Queries:**
- "What's the weather like?"
- "Show me the latest news"
- "Are there any weather alerts?"
- "Tell me about this location"

## Architecture

```
src/
├── lib.rs          # Shared types and constants
├── main.rs         # CLI entry point
├── geocoding.rs    # Location lookup service
├── weather.rs      # Weather API integration
├── news.rs         # News API integration
├── alerts.rs       # Weather alerts
├── location.rs     # Location details
└── llm.rs          # LLM integration
```

## API Dependencies

- **Open-Meteo**: Free weather and geocoding APIs
- **NewsAPI**: News headlines (requires API key)
- **Google Gemini**: Natural language processing
- **Anthropic Claude**: Alternative LLM option

## Development

### Adding New Tools

1. Create a new module in `src/`
2. Implement the tool struct and methods
3. Add to `LLMProcessor` in `src/llm.rs`
4. Update system prompts

### Adding New LLM Providers

1. Add new API integration in `src/llm.rs`
2. Update CLI arguments in `src/main.rs`
3. Add environment variable checks

## License

MIT License - see LICENSE file for details.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## Support

- **Issues**: [GitHub Issues](https://github.com/rohittcodes/rust-agent-demo/issues)
