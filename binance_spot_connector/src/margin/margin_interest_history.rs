use crate::http::{request::Request, Credentials, Method};

/// `GET /sapi/v1/margin/interestHistory`
///
/// * Response in descending order
/// * If `isolatedSymbol` is not sent, crossed margin data will be returned
/// * Set `archived` to `true` to query data from 6 months ago
/// * `type` in response has 4 enums:
///   - `PERIODIC` interest charged per hour
///   - `ON_BORROW` first interest charged on borrow
///   - `PERIODIC_CONVERTED` interest charged per hour converted into BNB
///   - `ON_BORROW_CONVERTED` first interest charged on borrow converted into BNB
///
/// Weight(IP): 1
///
/// # Example
///
/// ```
/// use binance_spot_connector::margin;
///
/// let request = margin::margin_interest_history().asset("BNB").current(1).size(100);
/// ```
pub struct MarginInterestHistory {
    asset: Option<String>,
    isolated_symbol: Option<String>,
    start_time: Option<u64>,
    end_time: Option<u64>,
    current: Option<u64>,
    size: Option<u64>,
    archived: Option<bool>,
    recv_window: Option<u64>,
    credentials: Option<Credentials>,
}

impl MarginInterestHistory {
    pub fn new() -> Self {
        Self {
            asset: None,
            isolated_symbol: None,
            start_time: None,
            end_time: None,
            current: None,
            size: None,
            archived: None,
            recv_window: None,
            credentials: None,
        }
    }

    pub fn asset(mut self, asset: &str) -> Self {
        self.asset = Some(asset.to_owned());
        self
    }

    pub fn isolated_symbol(mut self, isolated_symbol: &str) -> Self {
        self.isolated_symbol = Some(isolated_symbol.to_owned());
        self
    }

    pub fn start_time(mut self, start_time: u64) -> Self {
        self.start_time = Some(start_time);
        self
    }

    pub fn end_time(mut self, end_time: u64) -> Self {
        self.end_time = Some(end_time);
        self
    }

    pub fn current(mut self, current: u64) -> Self {
        self.current = Some(current);
        self
    }

    pub fn size(mut self, size: u64) -> Self {
        self.size = Some(size);
        self
    }

    pub fn archived(mut self, archived: bool) -> Self {
        self.archived = Some(archived);
        self
    }

    pub fn recv_window(mut self, recv_window: u64) -> Self {
        self.recv_window = Some(recv_window);
        self
    }

    pub fn credentials(mut self, credentials: &Credentials) -> Self {
        self.credentials = Some(credentials.clone());
        self
    }
}

impl From<MarginInterestHistory> for Request {
    fn from(request: MarginInterestHistory) -> Request {
        let mut params = vec![];

        if let Some(asset) = request.asset {
            params.push(("asset".to_owned(), asset));
        }

        if let Some(isolated_symbol) = request.isolated_symbol {
            params.push(("isolatedSymbol".to_owned(), isolated_symbol));
        }

        if let Some(start_time) = request.start_time {
            params.push(("startTime".to_owned(), start_time.to_string()));
        }

        if let Some(end_time) = request.end_time {
            params.push(("endTime".to_owned(), end_time.to_string()));
        }

        if let Some(current) = request.current {
            params.push(("current".to_owned(), current.to_string()));
        }

        if let Some(size) = request.size {
            params.push(("size".to_owned(), size.to_string()));
        }

        if let Some(archived) = request.archived {
            params.push(("archived".to_owned(), archived.to_string()));
        }

        if let Some(recv_window) = request.recv_window {
            params.push(("recvWindow".to_owned(), recv_window.to_string()));
        }

        Request {
            path: "/sapi/v1/margin/interestHistory".to_owned(),
            method: Method::Get,
            params,
            credentials: request.credentials,
            sign: true,
        }
    }
}

impl Default for MarginInterestHistory {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::MarginInterestHistory;
    use crate::http::{request::Request, Credentials, Method};

    static API_KEY: &str = "api-key";
    static API_SECRET: &str = "api-secret";

    #[test]
    fn margin_margin_interest_history_convert_to_request_test() {
        let credentials = Credentials::from_hmac(API_KEY.to_owned(), API_SECRET.to_owned());

        let request: Request = MarginInterestHistory::new()
            .asset("BNB")
            .current(1)
            .size(100)
            .recv_window(5000)
            .credentials(&credentials)
            .into();

        assert_eq!(
            request,
            Request {
                path: "/sapi/v1/margin/interestHistory".to_owned(),
                credentials: Some(credentials),
                method: Method::Get,
                params: vec![
                    ("asset".to_owned(), "BNB".to_string()),
                    ("current".to_owned(), "1".to_string()),
                    ("size".to_owned(), "100".to_string()),
                    ("recvWindow".to_owned(), "5000".to_string()),
                ],
                sign: true
            }
        );
    }
}
