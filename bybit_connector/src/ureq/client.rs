use crate::http::{request::Request, Credentials};
use crate::ureq::{Error, Response};
use crate::version::VERSION;
use http::Uri;
use std::time::{SystemTime, UNIX_EPOCH};
use ureq::{Agent, AgentBuilder, Error as UreqError};

#[derive(Clone)]
pub struct BybitHttpClient {
    client: Agent,
    base_url: String,
    timestamp_delta: i64,
    credentials: Option<Credentials>,
}

impl BybitHttpClient {
    pub fn new(client: Agent, base_url: &str) -> Self {
        Self {
            client,
            base_url: base_url.to_owned(),
            timestamp_delta: 0,
            credentials: None,
        }
    }

    pub fn with_url(base_url: &str) -> Self {
        Self {
            client: AgentBuilder::new().build(),
            base_url: base_url.to_owned(),
            timestamp_delta: 0,
            credentials: None,
        }
    }

    pub fn credentials(mut self, credentials: Credentials) -> Self {
        self.credentials = Some(credentials);
        self
    }

    pub fn timestamp_delta(mut self, timestamp_delta: i64) -> Self {
        self.timestamp_delta = timestamp_delta;
        self
    }
}

impl BybitHttpClient {
    pub fn send<R: Into<Request>>(&self, request: R) -> Result<Response, Box<Error>> {
        let Request {
            method,
            path,
            params,
            credentials,
            sign,
        } = request.into();

        // Build URL
        let url: Uri = format!("{}{}", self.base_url, path).parse()?;

        let mut ureq_request = self.client.request(method.as_ref(), &url.to_string());

        // Set User-Agent in header
        let user_agent = &format!("bybit-connector/{}", VERSION);
        ureq_request = ureq_request.set("User-Agent", user_agent);

        // Map query parameters
        let has_params = !params.is_empty();
        if has_params {
            for (k, v) in params.iter() {
                ureq_request = ureq_request.query(k, v);
            }
        }

        let client_credentials = self.credentials.as_ref();
        let request_credentials = credentials.as_ref();
        if let Some(Credentials { api_key, signature }) = request_credentials.or(client_credentials)
        {
            let mut timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Clock may have gone backwards")
            .as_millis();

            // Append timestamp delta to sync up with server time.
            timestamp -= self.timestamp_delta as u128;

            // Set API-Key in header

            if sign {
                // Use system clock, panic if system clock is behind `std::time::UNIX_EPOCH`
               
                
                // Stringfy available query parameters and append back to query parameters
                let signature = crate::utils::sign(
                    ureq_request
                        .request_url()
                        .unwrap()
                        .as_url()
                        .query()
                        .unwrap(),
                    signature,
                )
                .map_err(|_| Error::InvalidApiSecret)?;
                ureq_request = ureq_request.set("X-BAPI-API-KEY", api_key);
                ureq_request = ureq_request.set("X-BAPI-TIMESTAMP", &timestamp.to_string());
                ureq_request = ureq_request.set("X-BAPI-SIGN", &signature);
                
            }
        }

        log::debug!("{}", ureq_request.url());

        let response = match ureq_request.call() {
            Ok(response) => Ok(response),
            Err(UreqError::Status(_, response)) => Ok(response),
            Err(err) => Err(Error::Send(err)),
        }?;

        log::debug!("{}", response.status());

        Ok(Response::from(response))
    }
}

impl Default for BybitHttpClient {
    fn default() -> Self {
        Self::new(AgentBuilder::new().build(), &crate::config::rest_api(),)
    }
}