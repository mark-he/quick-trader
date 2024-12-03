use crate::http::{request::Request, Method};
use strum::Display;

#[derive(Debug, Display)]
pub enum AccountType {
    Unified,
    Contract,
    Spot,
}

pub struct AccountBalanceQueryRequest {
    pub account_type: AccountType,
    pub coin: Option<String>,
}

impl AccountBalanceQueryRequest {
    pub fn new(account_type: AccountType) -> Self {
        Self {
            account_type,
            coin: None,
        }
    }

    pub fn coin(mut self, coin: &str) -> Self {
        self.coin = Some(coin.to_owned());
        self
    }

    pub fn get_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("accountType".to_owned(), self.account_type.to_string()));

        if let Some(coin) = &self.coin {
            params.push(("coin".to_owned(), coin.clone()));
        }
        params
    }
}

impl From<AccountBalanceQueryRequest> for Request {
    fn from(request: AccountBalanceQueryRequest) -> Request {
        let params = request.get_params();
        Request {
            path: "/v5/account/wallet-balance".to_owned(),
            method: Method::Get,
            params,
            credentials: None,
            sign: true,
            body: "".to_owned(),
            recv_window: 5000,
        }
    }
}