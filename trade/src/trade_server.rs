
use common::{error::AppError, msmc::{EventTrait, Subscription}};

pub trait SymbolRoute {
    fn get_symbol(&self) -> String;
}

pub trait TradeServer {
    type Event: EventTrait + SymbolRoute + 'static;
    type OrderRequest;
    type CancelOrderRequest;
    type Position;
    type Account;
    type SymbolConfig;
    type SymbolInfo;
    fn init(&mut self) -> Result<(), AppError>;
    fn start(&mut self) -> Result<Subscription<Self::Event>, AppError>;
    fn new_order(&mut self, request: Self::OrderRequest) -> Result<(), AppError>;
    fn cancel_order(&mut self, requset: Self::CancelOrderRequest) -> Result<(), AppError>;
    fn cancel_orders(&mut self, symbol: &str) -> Result<(), AppError>;
    fn get_positions(&self, symbol: &str) -> Vec<Self::Position>;
    fn get_account(&self, account_id: &str) -> Option<Self::Account>;
    fn init_symbol(&self, symbol: &str, config: Self::SymbolConfig)-> Result<Self::SymbolInfo, AppError>;
    fn close(&self);
}

