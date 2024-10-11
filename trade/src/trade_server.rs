
use common::{error::AppError, msmc::Subscription};

#[derive(Debug, Clone)]
pub enum TradeData {
    Connected,
    UserLogin(TradeSession),
    UserLogout,
    SettlementConfirmed,
    OnOrder(Order),
    OnTrade(Trade),
    OrderQuery(Vec<Order>),
    TradeQuery(Vec<Trade>),
    PositionQuery(Vec<Position>),
    AccountQuery(Account),
    HeartBeatWarning(i32),
    Disconnected(i32),
    Error(i32, String),
}

pub trait TradeServer {
    fn session(&self) -> Option<TradeSession>;
    fn connect(&mut self, config: &TradeConfig) -> Result<Subscription<(i32, TradeData)>, AppError>;
    fn send_order(&mut self, order : &OrderInsert, unit_id: &str, request_id : i32);
    fn cancel_order(&mut self, action: &OrderAction, request_id : i32);
    fn get_positions(&self, unit_id: &str, symbol: &str) -> Vec<Position>;
    fn get_account(&self, unit_id: &str) -> Account;
}

#[derive(Debug, Clone)]
pub struct TradeSession {
    pub trading_day: String,
    pub login_time: String,
    pub session_id: i32,
}

#[derive(Debug, Clone, Default)]
pub struct TradeConfig {
    pub front_addr: String,
    pub broker_id: String,
    pub auth_code: String,
    pub app_id: String,
    pub user_id: String,
    pub password: String,
}

#[derive(Debug, Clone)]
pub struct OrderInsert {
    pub symbol: String,
    pub order_ref: String,
    pub offset: String,
    pub order_type: String,
    
    pub exchange_id: String,
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

#[derive(Debug, Clone, Default)]
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

#[derive(Debug, Clone)]
pub struct Position {
    pub symbol : String,
    pub position: u32,
    pub today_position: u32,
    pub direction: String,
    pub cost: f64,
    pub cost_offset: f64,
    pub trading_day: String,
    pub invest_unit_id : String,
}

#[derive(Debug, Clone, Default)]
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
