use std::str::FromStr;

use serde::{Deserialize, Serialize};
use trade::trade_server::SymbolRoute;


impl SymbolRoute for TradeEvent {
    fn get_symbol(&self) -> String {
        match self {
            TradeEvent::OnOrder(event) => {
                event.symbol.to_string()
            },
            TradeEvent::OnTrade(event) => {
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
        let parts: Vec<&str> = s.split('@').collect();
        if parts.len() == 2 {
            Ok(Symbol {
                symbol: parts[0].to_string(),
                exchange_id: parts[1].to_string(),
            })
        } else {
            Err("Invalid input format. Expected symbol@exchange_id".to_string())
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
pub enum TradeEvent {
    Connected,
    UserLogin(Session),
    UserLogout,
    SettlementConfirmed,
    OnOrder(Order),
    OnTrade(Trade),
    OrderQuery(Vec<Order>),
    TradeQuery(Vec<Trade>),
    PositionQuery(Vec<Position>),
    AccountQuery(Account),
    SymbolQuery(SymbolInfo),
    HeartBeatWarning(i32),
    Disconnected(i32),
    Error(i32, String),
}
unsafe impl Send for TradeEvent {}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
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
}

#[derive(Debug, Clone)]
pub struct OrderAction {
    pub symbol: String,
    pub action_ref: i32,
    pub exchange_id: String,
    pub sys_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Order {
    pub order_ref: String,
    pub direction: String,
    pub offset: String,
    pub price: f64,
    pub volume_total_original: u32,   
    pub submit_status: String,
    pub order_type: String,
    pub sys_id: String,
    pub status: String,
    pub volume_traded: u32,
    pub volume_total: u32,
    pub status_msg: String,
    pub symbol: String,
    pub request_id: i32,
    pub invest_unit_id : String,
}

#[derive(Debug, Clone)]
pub struct Trade {
    pub order_ref: String,
    pub trade_id: String,
    pub sys_id: String,
    pub direction: String,
    pub offset: String,
    pub price: f64,
    pub volume: u32,
    pub datetime: String,
    pub symbol: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Position {
    pub symbol : String,
    pub position: u32,
    pub today_position: u32,
    pub direction: String,
    pub cost: f64,
    pub cost_offset: f64,
    pub invest_unit_id : String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Account {
    pub account_id : String,
    pub interest: f64,
    pub balance: f64,
    pub available: f64,
}

impl Account {
    pub fn new(account_id : String, balance : f64) -> Self {
        Account {account_id, balance, interest: balance, available: balance}
    }
}
