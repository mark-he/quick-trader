use common::{error::AppError, msmc::Subscription};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize,)]
pub struct Tick {
    pub symbol: String,
    pub datetime: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
    pub turnover: f64,
    pub bids: Vec<Vec<f64>>,
    pub asks: Vec<Vec<f64>>,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize,)]
pub struct KLine {
    pub symbol: String,
    pub interval: String,
    pub datetime: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
    pub turnover: f64,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize,)]
pub struct Depth {
    pub symbol: String,
    pub bids: Vec<Vec<f64>>,
    pub asks: Vec<Vec<f64>>,
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

unsafe impl Send for MarketData {   
}
pub trait MarketServer {
    type Symbol: ToString + Clone;
    fn init(&mut self) -> Result<(), AppError>;
    fn start(&mut self) -> Result<Subscription<MarketData>, AppError>;
    fn subscribe_tick(&mut self, symbol: Self::Symbol) -> Result<(), AppError>;
    fn subscribe_kline(&mut self, symbol: Self::Symbol, interval: &str) -> Result<(), AppError>;
    fn load_kline(&mut self, symbol: Self::Symbol, interval: &str, count: u32) -> Result<Vec<KLine>, AppError>;
    fn get_server_ping(&self) -> usize;
    fn close(&self);
}