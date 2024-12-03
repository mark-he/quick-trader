use crate::{enums::{Category, KlineInterval}, http::{request::Request, Method}};
use serde::{Serialize, Deserialize};
use serde_json::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetKlinesRequest {
    pub category: Category,
    pub symbol: String,
    pub interval: KlineInterval,
    pub start: Option<u64>,
    pub end: Option<u64>,
    pub limit: Option<u64>,
}

impl GetKlinesRequest {
    pub fn new(category: Category, symbol: &str, interval: KlineInterval) -> Self {
        Self {
            category: category.to_owned(),
            symbol: symbol.to_owned(),
            interval: interval.to_owned(),
            start: None,
            end: None,
            limit: None,
        }
    }

    pub fn start(mut self, start: u64) -> Self {
        self.start = Some(start);
        self
    }
    
    pub fn end(mut self, end: u64) -> Self {
        self.end = Some(end);
        self
    }
    
    pub fn limit(mut self, limit: u64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string(&self)
    }
}

impl From<GetKlinesRequest> for Request {
    fn from(request: GetKlinesRequest) -> Request {
        Request {
            path: "/v5/market/kline".to_owned(),
            method: Method::Post,
            params: vec![],
            credentials: None,
            sign: true,
            body: request.to_json().unwrap(),
            recv_window: 5000
        }
    }
}