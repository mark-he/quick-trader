use crate::http::{request::Request, Method};


/// `GET /fapi/v1/premiumIndex`
///
/// Premium index kline bars of a symbol. Klines are uniquely identified by their open time.
///
/// * If `startTime` and `endTime` are not sent, the most recent klines are returned.
///
/// Weight(IP): 1.
///
/// # Example
///
/// ```
/// use binance_spot_connector::market::{self, klines::KlineInterval};
///
/// let request = market::mark_price_klines("BTCUSDT", KlineInterval::Minutes1)
///     .start_time(1654079109000)
///     .end_time(1654079209000);
/// ```
pub struct PremiumIndex {
    symbol: String,
}

impl PremiumIndex {
    pub fn new(symbol: &str) -> Self {
        Self {
            symbol: symbol.to_owned(),
        }
    }
}

impl From<PremiumIndex> for Request {
    fn from(request: PremiumIndex) -> Request {
        let params = vec![
            ("symbol".to_owned(), request.symbol),
        ];


        Request {
            path: "/fapi/v1/premiumIndex".to_owned(),
            method: Method::Get,
            params,
            credentials: None,
            sign: false,
        }
    }
}
