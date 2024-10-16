use crate::http::{request::Request, Method};

/// `GET /fapi/v2/ticker/price`
///
/// Latest price for a symbol or symbols.
///
/// * If the symbol is not sent, prices for all symbols will be returned in an array.
///
/// Weight(IP):
/// * `1` for a single symbol;
/// * `2` when the symbol parameter is omitted;
///
/// # Example
///
/// ```
/// use binance_spot_connector::market;
///
/// let request = market::ticker_price().symbol("BNBUSDT").symbols(vec!["BTCUSDT","BNBBTC"]);
/// ```
pub struct TickerPrice {
    symbol: Option<String>,
}

impl TickerPrice {
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

impl From<TickerPrice> for Request {
    fn from(request: TickerPrice) -> Request {
        let mut params = vec![];

        if let Some(symbol) = request.symbol {
            params.push(("symbol".to_owned(), symbol));
        }

        Request {
            path: "/fapi/v2/ticker/price".to_owned(),
            method: Method::Get,
            params,
            credentials: None,
            sign: false,
        }
    }
}

impl Default for TickerPrice {
    fn default() -> Self {
        Self::new()
    }
}