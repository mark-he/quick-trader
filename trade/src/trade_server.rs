
use common::{error::AppError, msmc::{EventTrait, Subscription}};

pub trait SymbolRoute {
    fn get_symbol(&self) -> String;
}

pub trait TradeServer {
    type Event: EventTrait + SymbolRoute + 'static;
    type OrderRequest;
    type Position;
    type Account;
    type SymbolConfig;
    fn connect(&mut self) -> Result<Subscription<Self::Event>, AppError>;
    fn new_order(&mut self, request: Self::OrderRequest) -> Result<(), AppError>;
    fn cancel_order(&mut self, symbol: &str, order_id: &str) -> Result<(), AppError>;
    fn cancel_orders(&mut self, symbol: &str) -> Result<(), AppError>;
    fn get_positions(&self, symbol: &str) -> Vec<Self::Position>;
    fn get_account(&self, account_id: &str) -> Option<Self::Account>;
    fn init_symbol(&self, symbol: &str, config: Self::SymbolConfig)-> Result<(), AppError>;
    fn close(self);
}

