use crate::http::{request::Request, Credentials, Method};

/// `GET /sapi/v1/margin/allAssets`
///
/// Weight(IP): 1
///
/// # Example
///
/// ```
/// use binance_spot_connector::margin;
///
/// let request = margin::margin_all_assets();
/// ```
pub struct MarginAllAssets {
    asset: Option<String>,
    credentials: Option<Credentials>,
}

impl MarginAllAssets {
    pub fn new() -> Self {
        Self {
            asset: None,
            credentials: None,
        }
    }

    pub fn asset(mut self, asset: &str) -> Self {
        self.asset = Some(asset.to_owned());
        self
    }

    pub fn credentials(mut self, credentials: &Credentials) -> Self {
        self.credentials = Some(credentials.clone());
        self
    }
}

impl From<MarginAllAssets> for Request {
    fn from(_request: MarginAllAssets) -> Request {
        let params = vec![];

        Request {
            path: "/sapi/v1/margin/allAssets".to_owned(),
            method: Method::Get,
            params,
            credentials: _request.credentials,
            sign: false,
        }
    }
}

impl Default for MarginAllAssets {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::MarginAllAssets;
    use crate::http::{request::Request, Credentials, Method};

    static API_KEY: &str = "api-key";
    static API_SECRET: &str = "api-secret";

    #[test]
    fn margin_margin_all_assets_convert_to_request_test() {
        let credentials = Credentials::from_hmac(API_KEY.to_owned(), API_SECRET.to_owned());

        let request: Request = MarginAllAssets::new().credentials(&credentials).into();

        assert_eq!(
            request,
            Request {
                path: "/sapi/v1/margin/allAssets".to_owned(),
                credentials: Some(credentials),
                method: Method::Get,
                params: vec![],
                sign: false
            }
        );
    }
}
