use crate::{enums::Category, http::{request::Request, Method}};
use serde::{Serialize, Deserialize};
use serde_json::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetMarginTypeRequest {
    pub category: Category,
    pub symbol: String,
    pub buy_leverage: String,
    pub sell_leverage: String,
    pub trade_mode: usize,
}

impl SetMarginTypeRequest {
    pub fn new(category: Category, symbol: &str, trade_mode: usize, buy_leverage: &str, sell_leverage: &str) -> Self {
        Self {
            category: category.to_owned(),
            symbol: symbol.to_owned(),
            trade_mode,
            buy_leverage: buy_leverage.to_owned(),
            sell_leverage: sell_leverage.to_owned(),
        }
    }
    
    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string(&self)
    }
}

impl From<SetMarginTypeRequest> for Request {
    fn from(request: SetMarginTypeRequest) -> Request {
        Request {
            path: "/v5/position/switch-isolated".to_owned(),
            method: Method::Post,
            params: vec![],
            credentials: None,
            sign: true,
            body: request.to_json().unwrap(),
            recv_window: 5000
        }
    }
}