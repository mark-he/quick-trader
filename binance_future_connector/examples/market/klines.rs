use binance_future_connector::{
    hyper::{BinanceHttpClient, Error},
    market::{self, klines::KlineInterval},
};
use env_logger::Builder;

#[tokio::main]
async fn main() -> Result<(), Error> {
    Builder::from_default_env()
        .filter(None, log::LevelFilter::Info)
        .init();

    let client = BinanceHttpClient::default();
    let request = market::klines("BNBUSDT", KlineInterval::Minutes1)
        .limit(1000);
    let data = client.send(request).await?.into_body_str().await?;
    log::info!("{}", data);
    Ok(())
}
