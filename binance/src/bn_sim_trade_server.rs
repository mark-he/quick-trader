use common::{error::AppError, msmc::Subscription};
use binance_future_connector::trade::{enums::{MarginAssetMode, PositionMode}, new_order::NewOrderRequest};
use trade::{sim_trade_server::{SimTradeConfig, SimTradeServer, SimNewOrderRequest}, trade_server::{Position, TradeEvent, TradeServer, Wallet}};
use crate::model::*;
pub struct BnSimTradeServer {
    pub inner: SimTradeServer,
}

impl BnSimTradeServer {
    pub fn new(config: SimTradeConfig) -> Self {
        let inner = SimTradeServer::new(config);
        BnSimTradeServer {
            inner,
        }
    }
}

impl TradeServer for BnSimTradeServer {
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
            position_side: request.position_side.unwrap().to_string(),
            order_type: request.type_.to_string(),
            reduce_only: if request.reduce_only.is_some() {"true" == request.reduce_only.unwrap()} else { false},
            quantity: request.quantity,
            price: request.price,
            new_client_order_id: request.new_client_order_id,
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
            margin_type: config.margin_type,
            dual_position_side: PositionMode::OneWayMode,
            multi_assets_margin: MarginAssetMode::SingleAsset,
            maint_margin_ratio: leverage_margin_ratio(config.leverage),
            quantity_precision: 8,
            price_precision: 8,
            quote_precision: 8,
        };
        Ok(symbol_info)
    }

    fn close(&self) {
    }
}

fn leverage_margin_ratio(leverage: i32) -> f64 {
    1.0 as f64 / leverage as f64 / 4.0 as f64
}

