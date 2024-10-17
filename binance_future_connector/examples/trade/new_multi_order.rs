use binance_future_connector::{
    http::Credentials,
    hyper::{BinanceHttpClient, Error},
    trade::{self, enums::Side},
};
use env_logger::Builder;
use rust_decimal_macros::dec;

#[tokio::main]
async fn main() -> Result<(), Error> {
    Builder::from_default_env()
        .filter(None, log::LevelFilter::Debug)
        .init();
    let credentials = Credentials::from_hmac("13d233877484f4ea87afbbb8c29e52072c4e4a4a8650fcd689e076fab082bdc6".to_owned(), "671b347de4235aa3c2d3d15664db16180593ab21f65f4826e54b8f8e1ba11395".to_owned());
    let client =
        BinanceHttpClient::default().credentials(credentials);
    let request = trade::new_multi_order().add(trade::new_order("BNBUSDT", Side::Sell, trade::enums::OrderType::Market).quantity(dec!(0.1)));
    let data = client.send(request).await?.into_body_str().await?;
    log::info!("{}", data);
    Ok(())
}
