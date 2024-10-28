use std::sync::{Arc, Mutex};

use binance::{bn_market_server::BnMarketServer, bn_trade_server::BnTradeServer};
use market::market_gateway::MarketGateway;
use trade::trade_gateway::TradeGateway;

type BnSharedMarketGw = Arc<Mutex<MarketGateway<BnMarketServer>>>;
type BnSharedTradeGw = Arc<Mutex<TradeGateway<BnTradeServer>>>;


pub struct Context {
    pub market_gateway: Option<BnSharedMarketGw>,
    pub trade_gateway: Option<BnSharedTradeGw>,
}

pub static mut CONTEXT: Context = Context {
    market_gateway: None,
    trade_gateway: None,
};

pub fn init(market_server: BnMarketServer, trade_server: BnTradeServer) {
    unsafe {
        CONTEXT.market_gateway = Some(Arc::new(Mutex::new(MarketGateway::new(market_server))));
        CONTEXT.trade_gateway = Some(Arc::new(Mutex::new(TradeGateway::new(trade_server))));
    }
}

pub fn get_market_gateway() -> BnSharedMarketGw {
    unsafe {
        CONTEXT.market_gateway.as_ref().unwrap().clone()
    }
}

pub fn get_trade_gateway() -> BnSharedTradeGw {
    unsafe {
        CONTEXT.trade_gateway.as_ref().unwrap().clone()
    }
}

