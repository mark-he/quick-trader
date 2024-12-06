use std::ffi::CString;

use binance_future_connector::{market_stream::enums::{Level, UpdateSpeed}, trade::enums::{MarginAssetMode, PositionMode}};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderEvent {
    pub symbol: String,
    pub client_order_id: String,
    pub side: String,
    pub order_type: String,
    pub original_quantity: f64,
    pub original_price: f64,
    pub average_price: f64,
    pub stop_price: f64,
    pub order_status: String,
    pub order_last_filled_quantity: f64,
    pub order_filled_accumulated_quantity: f64,
    pub last_filled_price: f64,
    pub order_trade_time: String,
    pub execution_type: String,
    pub trade_id: u64,
    pub is_reduce_only: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PositionEvent {
    pub symbol: String,
    pub position_amt: f64,
    pub entry_price: f64,
    pub breakeven_price: f64,
    pub accumulated_realized: f64,
    pub unrealized_profit: f64,
    pub margin_type: String,
    pub isolated_wallet: f64,
    pub position_side: String,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ServiceResult<T: Serialize> {
    pub error_code: i32,
    pub message: String,
    pub data: Option<T>,
}

impl <T: Serialize> ServiceResult<T> {
    pub fn new(error_code: i32, message: &str, data: Option<T>) -> Self {
        ServiceResult {
            error_code,
            message: message.to_string(),
            data,
        }
    }

    pub fn to_c_json(&self) -> Box<CString> {
        let json = serde_json::to_string(&self).unwrap();
        Box::new(CString::new(json).unwrap())
    }
}


#[derive(Debug, Clone, Serialize, Deserialize,)]
pub struct BacktestConfig {
    pub log_utc: bool,
    pub log_level: String,
    pub start_time: u64,
    pub end_time: u64,
    pub interval: u64,
    pub lines_per_sec: u64,
    pub asset: String,
    pub balance: u64,
    pub leverage: u64,
    pub order_completed_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize,)]
pub struct BnRealConfig {
    pub log_utc: bool,
    pub log_level: String,
    pub tick_update_speed: Option<UpdateSpeed>,
    pub depth_level: Level,
    pub api_key: String, 
    pub api_secret: String,
    pub dual_position_side: PositionMode,
    pub multi_assets_margin: MarginAssetMode,
}


#[derive(Debug, Clone, Serialize, Deserialize,)]
pub struct BnSimConfig {
    pub log_utc: bool,
    pub log_level: String,
    pub tick_update_speed: Option<UpdateSpeed>,
    pub depth_level: Level,
    pub asset: String,
    pub balance: u64,
    pub leverage: u64,
    pub order_completed_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize,)]
pub struct BbRealConfig {
    pub log_utc: bool,
    pub log_level: String,
    pub depth_level: u32,
    pub api_key: String, 
    pub api_secret: String,
    pub position_side: u32,
    pub settle_coin: String,
    pub margin_mode: String,
}

#[derive(Debug, Clone, Serialize, Deserialize,)]
pub struct BbSimConfig {
    pub log_utc: bool,
    pub log_level: String,
    pub depth_level: u32,
    pub asset: String,
    pub balance: u64,
    pub order_completed_status: String,
}