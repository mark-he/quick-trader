
use std::os::raw::*;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CTradeConfig {
    pub env: *const c_char,
    pub api_key: *const c_char,
    pub api_secret: *const c_char,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CKLine {
    pub datetime: *const c_char,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: i32,
    pub turnover: f64,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CTick {
    pub symbol: *const c_char,
    pub datetime: *const c_char,
    pub trading_day: *const c_char,
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
