use crate::http::{request::Request, Method};

pub struct AccountInfoRequest {
}

impl AccountInfoRequest {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn get_params(&self) -> Vec<(String, String)> {
        let params = Vec::new();
        params
    }
}

impl From<AccountInfoRequest> for Request {
    fn from(request: AccountInfoRequest) -> Request {
        let params = request.get_params();
        Request {
            path: "/v5/account/info".to_owned(),
            method: Method::Get,
            params,
            credentials: None,
            sign: true,
            body: "".to_owned(),
            recv_window: 5000,
        }
    }
}