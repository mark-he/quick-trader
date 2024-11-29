use bybit_connector::{
    config, http::Credentials, trade::{self, enums::{Category, Side, TimeInForceType}}, ureq::{BybitHttpClient, Error}
};

fn main() -> Result<(), Box<Error>> {
    config::enable_prod(false);
    println!("{:?}", serde_json::to_string(&Category::Linear));
    println!("{:?}", serde_json::to_string(&TimeInForceType::IOC));
    let credentials = Credentials::from_hmac("NNkjh5G30XGwYBpTLK".to_owned(), "0vo14u6XlB3WSvDVhY4YuaCXErONIETSQnfV".to_owned());
    let client = BybitHttpClient::default().credentials(credentials);
    let request = trade::new_order(Category::Linear, "BTCUSDT", Side::Buy, trade::enums::OrderType::Limit, "1");
    let data = client.send(request)?.into_body_str()?;
    println!("{:?}", data);
    Ok(())
}
