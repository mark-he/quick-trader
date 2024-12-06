use crate::bn_market_server::convert_json_to_k_lines;
use crate::model;
use binance_future_connector::market::klines::KlineInterval;
use binance_future_connector::ureq::BinanceHttpClient;
use binance_future_connector::market as bn_market;

use common::error::AppError;
use market::market_server::{KLine, MarketData, MarketServer};
use common::msmc::*;
use market::sim_market_server::{KLineLoader, SimMarketConfig, SimMarketServer};
use std::str::FromStr;

pub struct BnKlineLoader {

}

impl KLineLoader for BnKlineLoader {
    fn load_kline(&self, symbol: &str, interval: &str, count: u32, start_time: Option<u64>, end_time: Option<u64>) -> Result<Vec<KLine>, AppError> {
        let client = BinanceHttpClient::default();
        let kline_interval = KlineInterval::from_str(interval).map_err(|e| {AppError::new(-200, &e)})?;
        let mut request = bn_market::klines(&symbol, kline_interval).limit(count);
        if let Some(s) = start_time {
            request = request.start_time(s);
        }
        if let Some(s) = end_time {
            request = request.end_time(s);
        }
        let data = model::get_resp_result(client.send(request), vec![])?;
        let klines = convert_json_to_k_lines(&symbol, interval, &data).map_err(|e| AppError::new(-200, format!("{:?}", e).as_str()))?;
        Ok(klines)
    }
}

pub struct BnSimMarketServer {
    inner: SimMarketServer,
}

impl BnSimMarketServer {
    pub fn new(config: SimMarketConfig) -> Self {
        let inner = SimMarketServer::new(config, Box::new(BnKlineLoader {}));
        BnSimMarketServer {
            inner,
        }
    }
}

impl MarketServer for BnSimMarketServer {
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
