use binance_future_connector::{
    hyper::{BinanceHttpClient, Error},
    market,
};
use env_logger::Builder;

#[tokio::main]
async fn main() -> Result<(), Error> {
    Builder::from_default_env()
        .filter(None, log::LevelFilter::Info)
        .init();

    let client = BinanceHttpClient::default();
    let request = market::ticker_twenty_four_hr()
        .symbol("BNBUSDT");
    let data = client.send(request).await?.into_body_str().await?;
    log::info!("{}", data);
    Ok(())
}
