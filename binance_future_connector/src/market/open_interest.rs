use crate::http::{request::Request, Method};

/// `GET /fapi/v1/openInterest`
///
/// Query funding rate info for symbols that had FundingRateCap/ FundingRateFloor / fundingIntervalHours adjustment
/// ```
pub struct OpenInterest {
    symbol: String,
}

impl OpenInterest {
    pub fn new(symbol: &str) -> Self {
        Self {
            symbol: symbol.to_owned(),
        }
    }
}

impl From<OpenInterest> for Request {
    fn from(request: OpenInterest) -> Request {
        let params = vec![
            ("symbol".to_owned(), request.symbol),
        ];

        Request {
            path: "/fapi/v1/openInterest".to_owned(),
            method: Method::Get,
            params,
            credentials: None,
            sign: true,
        }
    }
}
