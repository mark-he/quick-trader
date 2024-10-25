
use common::{error::AppError, msmc::{EventTrait, Subscription}};

pub trait TradeServer {
    type Event: EventTrait + 'static;
    type OrderRequest;
    type Position;
    type Account;
    fn connect(&mut self) -> Result<Subscription<Self::Event>, AppError>;
    fn new_order(&mut self, request : Self::OrderRequest) -> Result<(), AppError>;
    fn cancel_order(&mut self, symbol: &str, order_id: &str) -> Result<(), AppError>;
    fn get_positions(&self, symbol: &str) -> Vec<Self::Position>;
    fn get_account(&self, account_id: &str) -> Option<Self::Account>;
    fn close(self);
}

