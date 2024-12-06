use bybit_connector::{
    config, http::Credentials, market, ureq::{BybitHttpClient, Error}
};

fn main() -> Result<(), Box<Error>> {
    config::enable_prod(false);
    let credentials = Credentials::from_hmac("NNkjh5G30XGwYBpTLK".to_owned(), "0vo14u6XlB3WSvDVhY4YuaCXErONIETSQnfV".to_owned());
    let client = BybitHttpClient::default().credentials(credentials);
    let data = client.send(market::klines(bybit_connector::enums::Category::Linear, "DOGEUSDT", bybit_connector::enums::KlineInterval::Minutes1))?.into_body_str()?;
    println!("{:?}", data);
    Ok(())
}
