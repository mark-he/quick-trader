use binance_future_connector::{
    http::Credentials, ureq::BinanceHttpClient, user_data_stream, wss_listen_key_keepalive::WssListeneKeyKeepalive
};
use env_logger::Builder;
use serde_json::Value;

fn main() {
    Builder::from_default_env()
        .filter(None, log::LevelFilter::Debug)
        .init();

    let keepalive = WssListeneKeyKeepalive::new(binance_future_connector::config::WSS_API).new_listen_key(|| {
        let credentials = Credentials::from_hmac("13d233877484f4ea87afbbb8c29e52072c4e4a4a8650fcd689e076fab082bdc6".to_owned(), "671b347de4235aa3c2d3d15664db16180593ab21f65f4826e54b8f8e1ba11395".to_owned());
        let client = BinanceHttpClient::default().credentials(credentials);
        let request = user_data_stream::new_listen_key();
        let ret = client.send(request);

        match ret {
            Ok(resp) => {
                let data =  resp.into_body_str();
                if let Ok(string_data) = data {
                    let json_value: Value = serde_json::from_str(&string_data).unwrap();
                    if let Some(key) = json_value.get("listenKey") {
                        return Some(key.as_str().unwrap().to_string());
                    }
                }
            },
            _ => {
            }
        }
        None
    }, 24 * 3600).renew_listen_key(|listen_key| {
        let credentials = Credentials::from_hmac("13d233877484f4ea87afbbb8c29e52072c4e4a4a8650fcd689e076fab082bdc6".to_owned(), "671b347de4235aa3c2d3d15664db16180593ab21f65f4826e54b8f8e1ba11395".to_owned());
        let client = BinanceHttpClient::default().credentials(credentials);
        let request = user_data_stream::renew_listen_key(listen_key);
        let ret = client.send(request);
    }, 3600)
    .stream(|m| {
        let data = m.into_data();
        let string_data = String::from_utf8(data).expect("Found invalid UTF-8 chars");
        log::info!("{}", &string_data);
        true
    });
}

