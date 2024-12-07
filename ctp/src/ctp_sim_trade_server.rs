use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;
use trade::sim_trade_server::{SimNewOrderRequest, SimTradeConfig, SimTradeServer};

use trade::trade_server::*;
use common::{msmc::Subscription, error::AppError};
use crate::model::{CancelOrderRequest, NewOrderRequest, Symbol, SymbolInfo};

use super::ctp_code::*;

pub struct CtpSimTradeServer {
    pub inner: SimTradeServer,
}

impl CtpSimTradeServer {
    pub fn new(config: SimTradeConfig) -> Self {
        let inner = SimTradeServer::new(config);
        CtpSimTradeServer {
            inner,
        }
    }
}

impl TradeServer for CtpSimTradeServer {
    type OrderRequest = NewOrderRequest;
    type CancelOrderRequest = CancelOrderRequest;
    type SymbolConfig = ();
    type SymbolInfo = SymbolInfo;
    type Symbol = Symbol;
    
    fn init(&mut self) -> Result<(), AppError> {
        self.inner.init()
    }
 
    fn start(&mut self) -> Result<Subscription<TradeEvent>, AppError> {
        self.inner.start()
    }
 
    fn new_order(&mut self, symbol: Symbol, request : NewOrderRequest) -> Result<(), AppError> {
        let sim_order = SimNewOrderRequest{
            symbol: symbol.symbol.clone(),
            side: request.direction.to_string(),
            position_side: request.direction.to_string(),
            order_type: request.order_type.to_string(),
            reduce_only:  request.offset == OFFSET_CLOSE.code,
            quantity: Decimal::from_u32(request.volume_total),
            price: Decimal::from_f64(request.limit_price as f64),
            new_client_order_id: Some(request.order_ref.clone()),
         };
         self.inner.new_order(symbol.symbol, sim_order)
    }
 
    fn cancel_order(&mut self, symbol: Symbol, request: CancelOrderRequest) -> Result<(), AppError> {
        self.inner.cancel_order(symbol.symbol, request.order_id)
    }
 
    fn cancel_orders(&mut self, symbol: Symbol) -> Result<(), AppError> {
        self.inner.cancel_orders(symbol.symbol)
    }
 
    fn get_positions(&self, symbol: Symbol) -> Result<Vec<Position>, AppError> {
        self.inner.get_positions(symbol.symbol)
    }
 
    fn get_account(&self, account_id: &str) -> Result<Option<Wallet>, AppError>{
        self.inner.get_account(account_id)
    } 

    fn init_symbol(&self, symbol:Symbol, _config: Self::SymbolConfig)-> Result<Self::SymbolInfo, AppError> {
        let symbol_info = SymbolInfo {
            symbol: symbol.symbol,
            margin_ratio: 0.1,
            underlying_multiple: 20 as f64,
            volume_multiple: 20 as f64,
            price_tick: 2 as f64,
        };
        Ok(symbol_info)
    }

    fn close(&self) {
        self.inner.close();
    }
}