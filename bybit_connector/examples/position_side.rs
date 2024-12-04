use bybit_connector::{
    config, http::Credentials, trade::{self},
    enums::{Category, Side},
    ureq::{BybitHttpClient, Error},
};

fn main() -> Result<(), Box<Error>> {
    config::enable_prod(false);
    let credentials = Credentials::from_hmac("NNkjh5G30XGwYBpTLK".to_owned(), "0vo14u6XlB3WSvDVhY4YuaCXErONIETSQnfV".to_owned());
    let client = BybitHttpClient::default().credentials(credentials);
    let request = trade::position_side(Category::Linear, 1).coin("").symbol("");
    let data = client.send(request)?.into_body_str()?;
    println!("{:?}", data);
    Ok(())
}
