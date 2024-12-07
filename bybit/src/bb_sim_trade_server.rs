use std::str::FromStr;

use common::{error::AppError, msmc::Subscription};
use bybit_connector::trade::new_order::NewOrderRequest;
use rust_decimal::Decimal;
use trade::{sim_trade_server::{SimTradeConfig, SimTradeServer, SimNewOrderRequest}, trade_server::{Position, TradeEvent, TradeServer, Wallet}};
use crate::model::*;
pub struct BbSimTradeServer {
    pub inner: SimTradeServer,
}

impl BbSimTradeServer {
    pub fn new(config: SimTradeConfig) -> Self {
        let inner = SimTradeServer::new(config);
        BbSimTradeServer {
            inner,
        }
    }
}

impl TradeServer for BbSimTradeServer {
    type OrderRequest = NewOrderRequest;
    type CancelOrderRequest = String;
    type SymbolConfig = SymbolConfig;
    type SymbolInfo = SymbolInfo;
    type Symbol = String;
    
    fn init(&mut self) -> Result<(), AppError> {
       self.inner.init()
    }

    fn start(&mut self) -> Result<Subscription<TradeEvent>, AppError> {
        self.inner.start()
    }

    fn new_order(&mut self, symbol: String, request : NewOrderRequest) -> Result<(), AppError> {
        let sim_order = SimNewOrderRequest{
            symbol: request.symbol.clone(),
            side: request.side.to_string(),
            position_side: request.position_idx.unwrap().to_string(),
            order_type: request.order_type.to_string(),
            reduce_only: if request.reduce_only.is_some() {request.reduce_only.unwrap()} else { false},
            quantity: Some(Decimal::from_str(&request.qty).map_err(|e| AppError::new(-200, &e.to_string()))?),
            price: request.price,
            new_client_order_id: request.order_link_id.clone(),
        };
        self.inner.new_order(symbol, sim_order)
    }

    fn cancel_order(&mut self, symbol: String, request: String) -> Result<(), AppError> {
        self.inner.cancel_order(symbol, request)
    }

    fn cancel_orders(&mut self, symbol: String) -> Result<(), AppError> {
        self.inner.cancel_orders(symbol)
    }

    fn get_positions(&self, symbol: String) -> Result<Vec<Position>, AppError> {
        self.inner.get_positions(symbol)
    }

    fn get_account(&self, account_id: &str) -> Result<Option<Wallet>, AppError>{
        self.inner.get_account(account_id)
    }
    
    fn init_symbol(&self, symbol: String, config: Self::SymbolConfig) -> Result<SymbolInfo, AppError> {
        let symbol_info = SymbolInfo {
            symbol: symbol.to_string(),
            leverage: config.leverage,
            margin_type: config.margin_type.to_string(),
            dual_position_side: "1".to_string(),
        };
        Ok(symbol_info)
    }

    fn close(&self) {
    }
}


