use crate::http::{request::Request, Method};
use serde::{Serialize, Deserialize};
use serde_json::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetMarginModeRequest {
    pub set_margin_mode: String,
}

impl SetMarginModeRequest {
    pub fn new(set_margin_mode: &str) -> Self {
        Self {
            set_margin_mode: set_margin_mode.to_owned(),
        }
    }
    
    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string(&self)
    }
}

impl From<SetMarginModeRequest> for Request {
    fn from(request: SetMarginModeRequest) -> Request {
        Request {
            path: "/v5/account/set-margin-mode".to_owned(),
            method: Method::Post,
            params: vec![],
            credentials: None,
            sign: true,
            body: request.to_json().unwrap(),
            recv_window: 5000
        }
    }
}