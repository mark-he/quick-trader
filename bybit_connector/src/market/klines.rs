use crate::{enums::{Category, KlineInterval}, http::{request::Request, Method}};
use serde::{Serialize, Deserialize};

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


    pub fn get_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("category".to_owned(), self.category.to_string()));
        params.push(("symbol".to_owned(), self.symbol.clone()));
        params.push(("interval".to_owned(), self.interval.to_string()));

        if let Some(start) = &self.start {
            params.push(("start".to_owned(), start.to_string()));
        }

        if let Some(end) = &self.end {
            params.push(("end".to_owned(), end.to_string()));
        }

        if let Some(limit) = self.limit {
            params.push(("limit".to_owned(), limit.to_string()));
        }
        params
    }
}

impl From<GetKlinesRequest> for Request {
    fn from(request: GetKlinesRequest) -> Request {
        let params = request.get_params();
        Request {
            path: "/v5/market/kline".to_owned(),
            method: Method::Get,
            params,
            credentials: None,
            sign: true,
            body: "".to_string(),
            recv_window: 5000
        }
    }
}