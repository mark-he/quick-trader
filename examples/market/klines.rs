use binance_spot_connector::{
    hyper::{BinanceHttpClient, Error},
    market::{self, klines::KlineInterval},
};
use env_logger::Builder;

#[tokio::main]
async fn main() -> Result<(), Error> {
    Builder::from_default_env()
        .filter(None, log::LevelFilter::Info)
        .init();

    let client = BinanceHttpClient::with_url("https://testnet.binance.vision");
    let request = market::klines("BNBUSDT", KlineInterval::Hours1)
        .start_time(1728614445000)
        .end_time(1728628845000);
    let data = client.send(request).await?.into_body_str().await?;
    log::info!("{}", data);
    Ok(())
}
