use bybit_connector::{
    config,
    http::{request::RequestBuilder, Credentials, Method},
    ureq::{BybitHttpClient, Error},
};

fn main() -> Result<(), Box<Error>> {
    config::enable_prod(false);

    let credentials = Credentials::from_hmac("NNkjh5G30XGwYBpTLK".to_owned(), "0vo14u6XlB3WSvDVhY4YuaCXErONIETSQnfV".to_owned());
    let client = BybitHttpClient::default().credentials(credentials);
    let request = RequestBuilder::new(Method::Post, "/v5/order/create").body("").sign();
    let data = client.send(request)?.into_body_str()?;
    print!("======================= {:?}", data);
    Ok(())
}
