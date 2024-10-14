use binance_spot_connector::{
    hyper::{BinanceHttpClient, Error},
    market,
};
use env_logger::Builder;

#[tokio::main]
async fn main() -> Result<(), Error> {
    Builder::from_default_env()
        .filter(None, log::LevelFilter::Info)
        .init();
    let client = BinanceHttpClient::with_url("https://testnet.binance.vision");

    //let credentials = Credentials::from_hmac("0i7Y5L3QfOsVoQohkNw3J5M5oF7wNzz0B4jNxSTAXG2Vi4YPXBRezBtC9EQFIBRF".to_owned(), "Aw9sIqM8vRl0Zn4H2hMkVkwfxZuTceYdWO3ohSjPpiL5mVowmuaEqUkZWDkpURN8".to_owned());
    //let client = BinanceHttpClient::default().credentials(credentials);
    let request = market::ping();
    let data = client.send(request).await?.into_body_str().await?;
    log::info!("{}", data);
    Ok(())
}
