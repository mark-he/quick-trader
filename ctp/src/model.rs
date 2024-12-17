use std::str::FromStr;

use common::error::AppError;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;
use trade::trade_server::{Order, Position, SymbolRoute, Wallet};
use ureq::Response;


pub fn get_resp_result<T: DeserializeOwned>(ret: Result<Response, AppError>, ignore_result: bool) -> Result<Option<T>, AppError> {
    let err;
    match ret {
        Ok(resp) => {
            let ret2 = resp.into_string();
            match ret2 {
                Ok(data) => {
                    if ignore_result {
                        return Ok(None)
                    } else {
                        let mut json_value: Value = serde_json::from_str(&data).unwrap();
                        let result = json_value.get_mut("result");
                        let obj = serde_json::from_value::<T>(result.unwrap().take()).map_err(|e| AppError::new(-200, &e.to_string()))?;
                        return Ok(Some(obj))
                    }
                },
                Err(e) => {
                    err = e;
                },
            }
        },
        Err(e) => {
            return Err(e);
        },
    }
    Err(AppError::new(-200, format!("{:?}", err).as_str()))
}


impl SymbolRoute for ServerEvent {
    fn get_symbol(&self) -> String {
        match self {
            ServerEvent::OnOrder(event) => {
                event.symbol.to_string()
            },
            _ => {
                "".to_string()
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Symbol {
    pub symbol: String,
    pub exchange_id: String,
}

impl FromStr for Symbol {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('.').collect();
        if parts.len() == 2 {
            Ok(Symbol {
                exchange_id: parts[0].to_string(),
                symbol: parts[1].to_string(),
            })
        } else {
            Err("Invalid input format. Expected exchange.symbol".to_string())
        }
    }
}

impl ToString for Symbol {
    fn to_string(&self) -> String {
        format!("{}", self.symbol)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CancelOrderRequest {
    pub order_id: String,
}


#[derive(Debug, Clone)]
pub struct Session {
    pub session_id: i32,
    pub front_id: i32,
    pub trading_day: String,
}

#[derive(Debug, Clone)]
pub enum ServerEvent {
    Connected,
    UserLogin(Session),
    UserLogout,
    SettlementConfirmed,
    OnOrder(Order),
    OrderQuery(Vec<Order>),
    PositionQuery(Vec<Position>),
    AccountQuery(Wallet),
    SymbolQuery(SymbolInfo),
    HeartBeatWarning(i32),
    Disconnected(i32),
    Error(i32, String),
}
unsafe impl Send for ServerEvent {}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CtpConfig {
    pub log_level: String,
    pub flow_path: String,
    pub front_addr: String,
    pub nm_addr: String,
    pub user_info: String,
    pub product_info: String,
    pub auth_code: String,
    pub app_id: String,
    pub broker_id: String,
    pub user_id: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SymbolInfo {
    pub symbol: String,
    pub margin_ratio: f64,
    pub underlying_multiple: f64,
    pub volume_multiple: f64,
    pub price_tick: f64,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewOrderRequest {
    pub order_ref: String,
    pub offset: String,
    pub order_type: String,
    pub volume_total: u32,
    pub direction: String,
    pub limit_price: f64,
    pub stop_price: f64,
}

#[derive(Debug, Clone)]
pub struct OrderAction {
    pub symbol: String,
    pub action_ref: i32,
    pub exchange_id: String,
    pub sys_id: String,
}
