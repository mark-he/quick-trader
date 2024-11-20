
use std::fmt::Debug;

use common::{error::AppError, msmc::Subscription};

pub trait SymbolRoute {
    fn get_symbol(&self) -> String;
}

pub trait TradeServer {
    type Event: Clone + Debug + Send + SymbolRoute + 'static;
    type OrderRequest;
    type CancelOrderRequest;
    type Position;
    type Account;
    type SymbolConfig;
    type SymbolInfo;
    type Symbol: ToString;

    fn init(&mut self) -> Result<(), AppError>;
    fn start(&mut self) -> Result<Subscription<Self::Event>, AppError>;
    fn new_order(&mut self, symbol: Self::Symbol, request: Self::OrderRequest) -> Result<(), AppError>;
    fn cancel_order(&mut self, symbol: Self::Symbol, requset: Self::CancelOrderRequest) -> Result<(), AppError>;
    fn cancel_orders(&mut self, symbol: Self::Symbol) -> Result<(), AppError>;
    fn init_symbol(&self, symbol: Self::Symbol, config: Self::SymbolConfig)-> Result<Self::SymbolInfo, AppError>;
    fn get_positions(&self, symbol: Self::Symbol) -> Result<Vec<Self::Position>, AppError>;
    fn get_account(&self, account_id: &str) -> Result<Option<Self::Account>, AppError>;
    fn close(&self);
}

