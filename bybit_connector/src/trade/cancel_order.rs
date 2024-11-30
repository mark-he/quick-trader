use crate::{enums::Category, http::{request::Request, Method}};
use serde::{Serialize, Deserialize};
use serde_json::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderRequest {
    pub category: Category,
    pub symbol: String,
    pub order_id: Option<String>,
    pub order_link_id: Option<String>,
    pub order_filter: Option<String>,
}

impl CancelOrderRequest {
    pub fn new(category: Category, symbol: &str) -> Self {
        Self {
            category: category.to_owned(),
            symbol: symbol.to_owned(),
            order_id: None,
            order_link_id: None,
            order_filter: None,
        }
    }

    pub fn order_id(mut self, order_id: &str) -> Self {
        self.order_id = Some(order_id.to_owned());
        self
    }
    
    pub fn order_filter(mut self, order_filter: &str) -> Self {
        self.order_filter = Some(order_filter.to_owned());
        self
    }
    
    pub fn order_link_id(mut self, order_link_id: &str) -> Self {
        self.order_link_id = Some(order_link_id.to_owned());
        self
    }
    
    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string(&self)
    }
}

impl From<CancelOrderRequest> for Request {
    fn from(request: CancelOrderRequest) -> Request {
        Request {
            path: "/v5/order/cancel".to_owned(),
            method: Method::Post,
            params: vec![],
            credentials: None,
            sign: true,
            body: request.to_json().unwrap(),
            recv_window: 5000
        }
    }
}