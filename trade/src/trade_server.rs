
use std::fmt::Debug;
use common::{error::AppError, msmc::Subscription};
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug)]
pub enum TradeEvent {
    PositionUpdate(Position),
    AccountUpdate(Wallet),
    OrderUpdate(Order),
}

impl SymbolRoute for TradeEvent {
    fn get_symbol(&self) -> String {
        match self {
            TradeEvent::PositionUpdate(p) => {
                p.symbol.clone()
            },
            TradeEvent::OrderUpdate(o) => {
                o.symbol.clone()
            }
            _ => {
                "".to_string()
            }
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    pub symbol: String,
    pub position_side: String,
    pub side: String,
    pub amount: f64,
    pub cost: f64,
    pub today_amount: f64,
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.symbol == other.symbol &&
        self.position_side == other.position_side &&
        self.side == other.side &&
        self.amount == other.amount &&
        self.cost == other.cost &&
        self.today_amount == other.today_amount
    }
}

impl Eq for Position {}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Wallet {
    pub asset: String,
    pub balance: f64,
    pub available_balance: f64,
}

impl PartialEq for Wallet {
    fn eq(&self, other: &Self) -> bool {
        self.asset == other.asset &&
        self.balance == other.balance &&
        self.available_balance == other.available_balance
    }
}

impl Eq for Wallet {}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    pub order_id: String,
    pub client_order_id: String,
    pub order_type: String,
    pub symbol: String,
    pub status: String,
    pub price: f64,
    pub offset: String,
    pub traded: f64,
    pub total: f64,
    pub side: String,
    pub message: String,
    pub timestamp: u64,
}

pub trait SymbolRoute {
    fn get_symbol(&self) -> String;
}

pub trait TradeServer {
    type OrderRequest;
    type CancelOrderRequest;
    type SymbolConfig;
    type SymbolInfo;
    type Symbol: ToString;

    fn init(&mut self) -> Result<(), AppError>;
    fn start(&mut self) -> Result<Subscription<TradeEvent>, AppError>;
    fn new_order(&mut self, symbol: Self::Symbol, request: Self::OrderRequest) -> Result<(), AppError>;
    fn cancel_order(&mut self, symbol: Self::Symbol, request: Self::CancelOrderRequest) -> Result<(), AppError>;
    fn cancel_orders(&mut self, symbol: Self::Symbol) -> Result<(), AppError>;
    fn init_symbol(&self, symbol: Self::Symbol, config: Self::SymbolConfig)-> Result<Self::SymbolInfo, AppError>;
    fn get_positions(&self, symbol: Self::Symbol) -> Result<Vec<Position>, AppError>;
    fn get_account(&self, account_id: &str) -> Result<Option<Wallet>, AppError>;
    fn close(&self);
}

