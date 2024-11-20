
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
    fn new_order(&mut self, symbol: &str, request: Self::OrderRequest) -> Result<(), AppError>;
    fn cancel_order(&mut self, symbol: &str, requset: Self::CancelOrderRequest) -> Result<(), AppError>;
    fn cancel_orders(&mut self, symbol: &str) -> Result<(), AppError>;
    fn init_symbol(&self, symbol: &str, config: Self::SymbolConfig)-> Result<Self::SymbolInfo, AppError>;
    fn get_positions(&self, symbol: &str) -> Result<Vec<Self::Position>, AppError>;
    fn get_account(&self, account_id: &str) -> Result<Option<Self::Account>, AppError>;
    fn close(&self);
}

