use crate::http::{request::Request, Method};

/// `GET /fapi/v1/ticker/24hr`
///
/// 24 hour rolling window price change statistics. Careful when accessing this with no symbol.
///
/// * If the symbol is not sent, tickers for all symbols will be returned in an array.
///
/// Weight(IP):
/// * `1` for a single symbol;
/// * `40` when the symbol parameter is omitted;
///
/// # Example
///
/// ```
/// use binance_spot_connector::market;
///
/// let request = market::ticker_twenty_four_hr().symbol("BNBUSDT").symbols(vec!["BTCUSDT","BNBBTC"]);
/// ```
pub struct Ticker24hr {
    symbol: Option<String>,
}

impl Ticker24hr {
    pub fn new() -> Self {
        Self {
            symbol: None,
        }
    }

    pub fn symbol(mut self, symbol: &str) -> Self {
        self.symbol = Some(symbol.to_owned());
        self
    }
}

impl From<Ticker24hr> for Request {
    fn from(request: Ticker24hr) -> Request {
        let mut params = vec![];

        if let Some(symbol) = request.symbol {
            params.push(("symbol".to_owned(), symbol));
        }

        Request {
            path: "/fapi/v1/ticker/24hr".to_owned(),
            method: Method::Get,
            params,
            credentials: None,
            sign: false,
        }
    }
}

impl Default for Ticker24hr {
    fn default() -> Self {
        Self::new()
    }
}