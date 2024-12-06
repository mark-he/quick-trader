use crate::bb_market_server::convert_json_to_k_lines;
use crate::model::{self, KlineQueryResp};
use bybit_connector::{enums::KlineInterval, ureq::BybitHttpClient};
use bybit_connector::market as bb_market;

use common::error::AppError;
use market::market_server::{KLine, MarketData, MarketServer};
use common::msmc::*;
use market::sim_market_server::{KLineLoader, SimMarketConfig, SimMarketServer};
use std::str::FromStr;

pub struct BbKlineLoader {

}

impl KLineLoader for BbKlineLoader {
    fn load_kline(&self, symbol: &str, interval: &str, count: u32, start_time: Option<u64>, end_time: Option<u64>) -> Result<Vec<KLine>, AppError> {
        let client = BybitHttpClient::default();
        let kline_interval = KlineInterval::from_str(interval).map_err(|e| {AppError::new(-200, &e)})?;
        let mut request = bb_market::klines(bybit_connector::enums::Category::Linear, &symbol, kline_interval).limit(count as u64);
        if let Some(s) = start_time {
            request = request.start(s);
        }
        if let Some(s) = end_time {
            request = request.end(s);
        }
        let mut data = model::get_resp_result::<KlineQueryResp>(client.send(request), vec![], false)?;
        let klines = convert_json_to_k_lines(&symbol, interval, data.take().unwrap()).map_err(|e| AppError::new(-200, format!("{:?}", e).as_str()))?;
        Ok(klines)
    }
}

pub struct BbSimMarketServer {
    inner: SimMarketServer,
}

impl BbSimMarketServer {
    pub fn new(config: SimMarketConfig) -> Self {
        let inner = SimMarketServer::new(config, Box::new(BbKlineLoader {}));
        BbSimMarketServer {
            inner,
        }
    }
}

impl MarketServer for BbSimMarketServer {
    type Symbol = String;
    
    fn load_kline(&mut self, symbol: String, interval: &str, count: u32) -> Result<Vec<KLine>, AppError> {
        self.inner.load_kline(symbol, interval, count)
    }

    fn subscribe_tick(&mut self, symbol: String) -> Result<(), AppError>{
        self.inner.subscribe_tick(symbol)
    }

    fn subscribe_kline(&mut self, symbol: String, interval: &str) -> Result<(), AppError>{
        self.inner.subscribe_kline(symbol, interval)
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
