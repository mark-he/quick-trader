use crate::http::{request::Request, Method};
use super::klines::KlineInterval;


/// `GET /fapi/v1/indexPriceKlines`
///
/// Kline/candlestick bars for the index price of a pair. Klines are uniquely identified by their open time.
///
/// * If `startTime` and `endTime` are not sent, the most recent klines are returned.
///
/// Weight(IP): based on parameter LIMIT, 5 by default.
///
/// # Example
///
/// ```
/// use binance_spot_connector::market::{self, klines::KlineInterval};
///
/// let request = market::index_price_klines("BTCUSDT", KlineInterval::Minutes1)
///     .start_time(1654079109000)
///     .end_time(1654079209000);
/// ```
pub struct IndexPriceKlines {
    pair: String,
    interval: KlineInterval,
    start_time: Option<u64>,
    end_time: Option<u64>,
    limit: Option<u32>,
}

impl IndexPriceKlines {
    pub fn new(pair: &str, interval: KlineInterval) -> Self {
        Self {
            pair: pair.to_owned(),
            interval,
            start_time: None,
            end_time: None,
            limit: None,
        }
    }

    pub fn start_time(mut self, start_time: u64) -> Self {
        self.start_time = Some(start_time);
        self
    }

    pub fn end_time(mut self, end_time: u64) -> Self {
        self.end_time = Some(end_time);
        self
    }

    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
}

impl From<IndexPriceKlines> for Request {
    fn from(request: IndexPriceKlines) -> Request {
        let mut params = vec![
            ("pair".to_owned(), request.pair),
            ("interval".to_owned(), request.interval.to_string()),
        ];

        if let Some(start_time) = request.start_time {
            params.push(("startTime".to_owned(), start_time.to_string()));
        }

        if let Some(end_time) = request.end_time {
            params.push(("endTime".to_owned(), end_time.to_string()));
        }

        if let Some(limit) = request.limit {
            params.push(("limit".to_owned(), limit.to_string()));
        }

        Request {
            path: "/fapi/v1/indexPriceKlines".to_owned(),
            method: Method::Get,
            params,
            credentials: None,
            sign: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{KlineInterval, IndexPriceKlines};
    use crate::http::{request::Request, Method};

    #[test]
    fn market_kline_candlestick_data_convert_to_request_test() {
        let request: Request = IndexPriceKlines::new("BTCUSDT", KlineInterval::Minutes1)
            .start_time(1654079109000)
            .end_time(1654079209000)
            .limit(100)
            .into();

        assert_eq!(
            request,
            Request {
                path: "/fapi/v1/indexPriceKlines".to_owned(),
                credentials: None,
                method: Method::Get,
                params: vec![
                    ("pair".to_owned(), "BTCUSDT".to_string()),
                    ("interval".to_owned(), "1m".to_string()),
                    ("startTime".to_owned(), "1654079109000".to_string()),
                    ("endTime".to_owned(), "1654079209000".to_string()),
                    ("limit".to_owned(), "100".to_string())
                ],
                sign: false
            }
        )
    }
}
