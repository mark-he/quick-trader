use crate::{enums::Category, http::{request::Request, Method}};
use serde::{Serialize, Deserialize};
use serde_json::Result;


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrdersRequest {
    pub category: Category,
    pub symbol: String,
    pub base_coin: Option<String>,
    pub settle_coin: Option<String>,
    pub stop_order_type: Option<String>,
    pub order_filter: Option<String>,
}

impl CancelOrdersRequest {
    pub fn new(category: Category, symbol: &str) -> Self {
        Self {
            category: category.to_owned(),
            symbol: symbol.to_owned(),
            base_coin: None,
            settle_coin: None,
            stop_order_type: None,
            order_filter: None,
        }
    }

    pub fn base_coin(mut self, base_coin: &str) -> Self {
        self.base_coin = Some(base_coin.to_owned());
        self
    }

    pub fn stop_order_type(mut self, stop_order_type: &str) -> Self {
        self.stop_order_type = Some(stop_order_type.to_owned());
        self
    }
    
    pub fn order_filter(mut self, order_filter: &str) -> Self {
        self.order_filter = Some(order_filter.to_owned());
        self
    }
    
    pub fn settle_coin(mut self, settle_coin: &str) -> Self {
        self.settle_coin = Some(settle_coin.to_owned());
        self
    }
    
    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string(&self)
    }
}

impl From<CancelOrdersRequest> for Request {
    fn from(request: CancelOrdersRequest) -> Request {
        Request {
            path: "/v5/order/cancel-all".to_owned(),
            method: Method::Post,
            params: vec![],
            credentials: None,
            sign: true,
            body: request.to_json().unwrap(),
            recv_window: 5000
        }
    }
}