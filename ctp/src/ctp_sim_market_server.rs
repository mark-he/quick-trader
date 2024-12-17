use common::error::AppError;
use market::sim_market_server::{SimMarketConfig, SimMarketServer};
use crate::{ctp_market_server::CtpKlineLoader, model::Symbol};

use market::market_server::{KLine, MarketData, MarketServer};
use common::msmc::*;


pub struct CtpSimMarketServer {
    pub inner: SimMarketServer,
}

impl CtpSimMarketServer {
    pub fn new(config: SimMarketConfig) -> Self {
        let inner = SimMarketServer::new(config, Box::new(CtpKlineLoader::new("http://127.0.0.1:5001")));
        CtpSimMarketServer {
            inner,
        }
    }
}

impl MarketServer for CtpSimMarketServer {
    type Symbol = Symbol;
    
    fn load_kline(&mut self, symbol: Symbol, interval: &str, count: u32) -> Result<Vec<KLine>, AppError> {
        self.inner.load_kline(symbol.symbol, interval, count)
    }

    fn subscribe_tick(&mut self, symbol: Symbol) -> Result<(), AppError>{
        self.inner.subscribe_tick(symbol.symbol)
    }

    fn subscribe_kline(&mut self, symbol: Symbol, interval: &str) -> Result<(), AppError>{
        self.inner.subscribe_kline(symbol.symbol, interval)
    }

    fn get_server_ping(&self) -> usize {
        self.inner.get_server_ping()
    }

    fn init(&mut self) -> Result<(), AppError> {
        self.inner.init()
    }

    fn start(&mut self) -> Result<Subscription<MarketData>, AppError> {
        self.inner.start()
    }

    fn close(&self) {
        self.inner.close()
    }
}