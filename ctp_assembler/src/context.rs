use std::sync::{Arc, Mutex};

use ctp::{ctp_market_server::CtpMarketServer, ctp_trade_server::CtpTradeServer};
use market::market_gateway::MarketGateway;
use trade::trade_gateway::TradeGateway;

type CtpSharedMarketGw = Arc<Mutex<MarketGateway<CtpMarketServer>>>;
type CtpSharedTradeGw = Arc<Mutex<TradeGateway<CtpTradeServer>>>;

pub struct Context {
    pub market_gateway: Option<CtpSharedMarketGw>,
    pub trade_gateway: Option<CtpSharedTradeGw>,
}

pub static mut CONTEXT: Context = Context {
    market_gateway: None,
    trade_gateway: None,
};

pub fn init(market_server: CtpMarketServer, trade_server: CtpTradeServer) {
    unsafe {
        CONTEXT.market_gateway = Some(Arc::new(Mutex::new(MarketGateway::new(market_server))));
        CONTEXT.trade_gateway = Some(Arc::new(Mutex::new(TradeGateway::new(trade_server))));
    }
}

pub fn get_market_gateway() -> CtpSharedMarketGw {
    unsafe {
        CONTEXT.market_gateway.as_ref().unwrap().clone()
    }
}

pub fn get_trade_gateway() -> CtpSharedTradeGw {
    unsafe {
        CONTEXT.trade_gateway.as_ref().unwrap().clone()
    }
}

