use crate::{enums::Category, http::{request::Request, Method}};
use serde::{Serialize, Deserialize};
use serde_json::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SwitchModeRequest {
    pub category: Category,
    pub symbol: Option<String>,
    pub coin: Option<String>,
    pub mode: u32,
}

impl SwitchModeRequest {
    pub fn new(category: Category, mode: u32) -> Self {
        Self {
            category: category.to_owned(),
            mode,
            symbol: Some("".to_string()),
            coin: Some("".to_string()),
        }
    }
    
    pub fn symbol(mut self, symbol: &str) -> Self {
        self.symbol = Some(symbol.to_owned());
        self
    }

    pub fn coin(mut self, coin: &str) -> Self {
        self.coin = Some(coin.to_owned());
        self
    }

    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string(&self)
    }
}

impl From<SwitchModeRequest> for Request {
    fn from(request: SwitchModeRequest) -> Request {
        Request {
            path: "/v5/position/switch-mode".to_owned(),
            method: Method::Post,
            params: vec![],
            credentials: None,
            sign: true,
            body: request.to_json().unwrap(),
            recv_window: 5000
        }
    }
}