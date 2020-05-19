use serde::{Deserialize, Serialize};
use tokio_postgres;
use tokio_postgres::{Error, NoTls};

#[derive(Serialize, Deserialize)]
struct DailyPrice {
    timestamp: String,
    open: f32,
    high: f32,
    low: f32,
    close: f32,
    adjusted_close: f32,
    volume: i32,
    dividend_amount: f32,
    split_coefficient: f32,
}

#[derive(Serialize, Deserialize)]
pub struct SymbolPrices {
    symbol: String,
    values: Vec<DailyPrice>,
}

pub async fn store_symbol_prices(prices: &SymbolPrices) -> Result<(), Error> {
    println!("Storing {}", &prices.symbol);

    let (client, connection) =
        tokio_postgres::connect("postgresql://postgres@localhost/markets", NoTls).await?; // TODO: creds to env vars

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    for symbol_price in prices.values.iter() {
        client
            .query(
                r#"
                    INSERT INTO
                        symbol_prices (id, symbol, open, high, low, close, close_adjusted, volume, dividend_amount, split_coefficient, date)
                    VALUES
                        (DEFAULT, $1, $2, $3, $4, $5, $6, $7, $8, $9, to_date($10, 'yyyy-mm-dd'))
                "#,
                &[
                    &prices.symbol,
                    &symbol_price.open,
                    &symbol_price.high,
                    &symbol_price.low,
                    &symbol_price.close,
                    &symbol_price.adjusted_close,
                    &symbol_price.volume,
                    &symbol_price.dividend_amount,
                    &symbol_price.split_coefficient,
                    &symbol_price.timestamp,
                ],
            )
            .await?;
    }

    Ok(())
}
