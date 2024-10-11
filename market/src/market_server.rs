use std::collections::HashMap;
use common::{error::AppError, msmc::Subscription};

use super::kline::KLine;

#[derive(Debug, Clone, Default)]
pub struct Tick {
    pub symbol: String,
    pub datetime: String,
    pub trading_day: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: i32,
    pub turnover: f64,
    pub open_interest: f64,
    pub last_price: f64,
    pub bid_price1: f64,
    pub bid_price2: f64,
    pub bid_price3: f64,
    pub bid_price4: f64,
    pub bid_price5: f64,
    pub bid_volume1: i32,
    pub bid_volume2: i32,
    pub bid_volume3: i32,
    pub bid_volume4: i32,
    pub bid_volume5: i32,
    pub ask_price1: f64,
    pub ask_price2: f64,
    pub ask_price3: f64,
    pub ask_price4: f64,
    pub ask_price5: f64,
    pub ask_volume1: i32,
    pub ask_volume2: i32,
    pub ask_volume3: i32,
    pub ask_volume4: i32,
    pub ask_volume5: i32,
}

#[derive(Debug, Clone)]
pub struct Event {
    pub code: i32,
    pub message: String,
}

#[derive(Debug, Clone)]
pub enum MarketData {
    Connected,
    UserLogin,
    TradeDayEnded(String),
    MarketClosed,
    Disconnected(i32),
    Tick(Tick),
    Kline(KLine),
    Error(i32, String),
}

pub trait MarketServer {
    fn connect(&mut self, prop : &HashMap<String, String>) -> Result<Subscription<MarketData>, AppError>;
    fn subscribe(&mut self, symbol: &str) -> Result<(), AppError>;
}