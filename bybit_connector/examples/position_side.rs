use bybit_connector::{
    config, http::Credentials, trade::{self},
    enums::Category,
    ureq::{BybitHttpClient, Error},
};

fn main() -> Result<(), Box<Error>> {
    config::enable_prod(false);
    let credentials = Credentials::from_hmac("NNkjh5G30XGwYBpTLK".to_owned(), "0vo14u6XlB3WSvDVhY4YuaCXErONIETSQnfV".to_owned());
    let client = BybitHttpClient::default().credentials(credentials);
    let data = client.send(trade::position_side(Category::Linear, 0))?.into_body_str()?;
    println!("{:?}", data);
    Ok(())
}
