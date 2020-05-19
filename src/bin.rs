use dotenv::dotenv;
use std::env;
use store::{store_symbol_prices, SymbolPrices};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let api_key = env::var("API_KEY").expect("API_KEY env var must be set");
    let symbol = env::var("SYMBOL").expect("SYMBOL env var must be set");

    let url = format!(
        "https://www.alphavantage.co/query?function={}&symbol={}&outputsize=full&&apikey={}",
        "TIME_SERIES_DAILY_ADJUSTED", &symbol, &api_key,
    );

    let prices = reqwest::get(&url).await?.json::<SymbolPrices>().await?;

    store_symbol_prices(&prices).await?;
    Ok(())
}
