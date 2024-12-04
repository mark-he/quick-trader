use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize,)]
pub struct SimMarketConfig {
    pub start_time: u64,
    pub end_time: u64,
    pub interval: u64,
    pub lines_per_sec: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize,)]
pub struct SimTradeConfig {
    pub order_completed_status: String,
    pub asset: String,
    pub balance: u64,
}
