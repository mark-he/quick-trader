
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
    pub datetime: [c_char; 100],
    pub symbol: [c_char; 100],
    pub interval: [c_char; 100],
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
    pub turnover: f64,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CTick {
    pub symbol: [c_char; 100],
    pub datetime: [c_char; 100],
    pub trading_day: [c_char; 100],
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
    pub turnover: f64,
    pub open_interest: f64,
    pub last_price: f64,
    pub bid_price1: f64,
    pub bid_price2: f64,
    pub bid_price3: f64,
    pub bid_price4: f64,
    pub bid_price5: f64,
    pub bid_volume1: f64,
    pub bid_volume2: f64,
    pub bid_volume3: f64,
    pub bid_volume4: f64,
    pub bid_volume5: f64,
    pub ask_price1: f64,
    pub ask_price2: f64,
    pub ask_price3: f64,
    pub ask_price4: f64,
    pub ask_price5: f64,
    pub ask_volume1: f64,
    pub ask_volume2: f64,
    pub ask_volume3: f64,
    pub ask_volume4: f64,
    pub ask_volume5: f64,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CNewOrderRequest {
    pub symbol: *const c_char,
    pub side: *const c_char,
    pub position_side: *const c_char,
    pub type_: *const c_char,
    pub reduce_only: *const c_char,
    pub quantity: f64,
    pub price: f64,
    pub new_client_order_id: *const c_char,
    pub stop_price: f64,
    pub close_position: *const c_char,
    pub activation_price: f64,
    pub callback_rate: f64,
    pub time_in_force: *const c_char,
    pub working_type: *const c_char,
    pub price_protect: *const c_char,
    pub new_order_resp_type: *const c_char,
    pub price_match: *const c_char,
    pub self_trade_prevention_mode: *const c_char,
    pub good_till_date: i64,
    pub recv_window: i64,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CPosition {
    pub symbol: *const c_char,
    pub initial_margin: f64,
    pub maint_margin: f64,
    pub unrealized_profit: f64,
    pub position_initial_margin: f64,
    pub open_order_initial_margin: f64,
    pub leverage: *const c_char,
    pub isolated: bool,
    pub entry_price: f64,
    pub max_notional: f64,
    pub bid_notional: f64,
    pub ask_notional: f64,
    pub position_side: *const c_char,
    pub position_amt: f64,
    pub update_time: u64,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CAsset {
    pub asset: *const c_char,
    pub wallet_balance: f64,
    pub unrealized_profit: f64,
    pub margin_balance: f64,
    pub maint_margin: f64,
    pub initial_margin: f64,
    pub position_initial_margin: f64,
    pub open_order_initial_margin: f64,
    pub cross_wallet_balance: f64,
    pub cross_unpnl: f64,
    pub available_balance: f64,
    pub max_withdraw_amount: f64,
    pub margin_available: bool,
    pub update_time: u64,
}

#[repr(C)]
pub struct CKLines {
    pub length: usize,
    pub ptr: *const CKLine,
}