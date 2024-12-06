use crate::http::{request::Request, Method};
use serde::{Serialize, Deserialize};
use serde_json::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetServerTimeRequest {
    
}

impl GetServerTimeRequest {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string(&self)
    }
}

impl From<GetServerTimeRequest> for Request {
    fn from(request: GetServerTimeRequest) -> Request {
        Request {
            path: "/v5/market/time".to_owned(),
            method: Method::Get,
            params: vec![],
            credentials: None,
            sign: true,
            body: request.to_json().unwrap(),
            recv_window: 5000
        }
    }
}