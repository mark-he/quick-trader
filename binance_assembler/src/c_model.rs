use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
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
    pub trade_id: u64,
}