use serde::{Deserialize, Serialize};

use crate::{enums::Category, http::{request::Request, Method}};


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionQueryRequest {
    pub category: Category,
    pub symbol: Option<String>,
    pub base_coin: Option<String>,
    pub settle_coin: Option<String>,
    pub limit: Option<usize>,
    pub cursor: Option<String>,
}

impl PositionQueryRequest {
    pub fn new(category: Category) -> Self {
        Self {
            category: category.to_owned(),
            symbol: None,
            base_coin: None,
            settle_coin: None,
            limit: None,
            cursor: None,
        }
    }

    pub fn symbol(mut self, symbol: &str) -> Self {
        self.symbol = Some(symbol.to_owned());
        self
    }

    pub fn base_coin(mut self, base_coin: &str) -> Self {
        self.base_coin = Some(base_coin.to_owned());
        self
    }

    pub fn settle_coin(mut self, settle_coin: &str) -> Self {
        self.settle_coin = Some(settle_coin.to_owned());
        self
    }

    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn cursor(mut self, cursor: &str) -> Self {
        self.cursor = Some(cursor.to_owned());
        self
    }

    pub fn get_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("category".to_owned(), self.category.to_string()));

        if let Some(symbol) = &self.symbol {
            params.push(("symbol".to_owned(), symbol.clone()));
        }

        if let Some(base_coin) = &self.base_coin {
            params.push(("baseCoin".to_owned(), base_coin.clone()));
        }

        if let Some(settle_coin) = &self.settle_coin {
            params.push(("settleCoin".to_owned(), settle_coin.clone()));
        }

        if let Some(limit) = self.limit {
            params.push(("limit".to_owned(), limit.to_string()));
        }

        if let Some(cursor) = &self.cursor {
            params.push(("cursor".to_owned(), cursor.clone()));
        }
        params
    }
}

impl From<PositionQueryRequest> for Request {
    fn from(request: PositionQueryRequest) -> Request {
        let params = request.get_params();
        Request {
            path: "/v5/position/list".to_owned(),
            method: Method::Get,
            params,
            credentials: None,
            sign: true,
            body: "".to_owned(),
            recv_window: 5000,
        }
    }
}