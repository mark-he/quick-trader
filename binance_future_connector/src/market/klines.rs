use crate::http::{request::Request, Method};
use strum::Display;

#[derive(Copy, Clone, Display)]
pub enum KlineInterval {
    #[strum(serialize = "1m")]
    Minutes1,
    #[strum(serialize = "3m")]
    Minutes3,
    #[strum(serialize = "5m")]
    Minutes5,
    #[strum(serialize = "15m")]
    Minutes15,
    #[strum(serialize = "30m")]
    Minutes30,
    #[strum(serialize = "1h")]
    Hours1,
    #[strum(serialize = "2h")]
    Hours2,
    #[strum(serialize = "4h")]
    Hours4,
    #[strum(serialize = "6h")]
    Hours6,
    #[strum(serialize = "8h")]
    Hours8,
    #[strum(serialize = "12h")]
    Hours12,
    #[strum(serialize = "1d")]
    Days1,
    #[strum(serialize = "3d")]
    Days3,
    #[strum(serialize = "1w")]
    Weeks1,
    #[strum(serialize = "1M")]
    Months1,
}

/// `GET /api/v3/klines`
///
/// Kline/candlestick bars for a symbol.
/// Klines are uniquely identified by their open time.
///
pub struct KlinesRequest {
    symbol: String,
    interval: KlineInterval,
    start_time: Option<u64>,
    end_time: Option<u64>,
    limit: Option<u32>,
}

impl KlinesRequest {
    pub fn new(symbol: &str, interval: KlineInterval) -> Self {
        Self {
            symbol: symbol.to_owned(),
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

impl From<KlinesRequest> for Request {
    fn from(request: KlinesRequest) -> Request {
        let mut params = vec![
            ("symbol".to_owned(), request.symbol),
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
            path: "/fapi/v1/klines".to_owned(),
            method: Method::Get,
            params,
            credentials: None,
            sign: false,
        }
    }
}
