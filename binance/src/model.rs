use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BinanceKline {
    /// Event type
    #[serde(rename = "e")]
    pub event_type: String,
    /// Event time
    #[serde(rename = "E")]
    pub event_time: u64,
    /// Symbol of the trading pair
    #[serde(rename = "s")]
    pub symbol: String,
    /// Kline data
    #[serde(rename = "k")]
    pub kline_data: KlineData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KlineData {
    /// Start time of the kline
    #[serde(rename = "t")]
    pub start_time: u64,
    /// Close time of the kline
    #[serde(rename = "T")]
    pub close_time: u64,
    /// Symbol of the trading pair
    #[serde(rename = "s")]
    pub symbol: String,
    /// Interval
    #[serde(rename = "i")]
    pub interval: String,
    /// First trade ID
    #[serde(rename = "f")]
    pub first_trade_id: u64,
    /// Last trade ID
    #[serde(rename = "L")]
    pub last_trade_id: u64,
    /// Open price
    #[serde(rename = "o")]
    pub open_price: String,
    /// Close price
    #[serde(rename = "c")]
    pub close_price: String,
    /// High price
    #[serde(rename = "h")]
    pub high_price: String,
    /// Low price
    #[serde(rename = "l")]
    pub low_price: String,
    /// Volume of the base asset
    #[serde(rename = "v")]
    pub base_asset_volume: String,
    /// Number of trades
    #[serde(rename = "n")]
    pub number_of_trades: u64,
    /// Whether the kline is closed
    #[serde(rename = "x")]
    pub is_closed: bool,
    /// Volume of the quote asset
    #[serde(rename = "q")]
    pub quote_asset_volume: String,
    /// Taker buy volume of the base asset
    #[serde(rename = "V")]
    pub taker_buy_base_asset_volume: String,
    /// Taker buy volume of the quote asset
    #[serde(rename = "Q")]
    pub taker_buy_quote_asset_volume: String,
    /// Ignore
    #[serde(rename = "B")]
    ignored_value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BinanceTick {
    #[serde(rename = "e")]
    /// Event type
    pub event_type: String,
    #[serde(rename = "E")]
    /// Event time
    pub event_time: u64,
    #[serde(rename = "s")]
    /// Symbol
    pub symbol: String,
    #[serde(rename = "c")]
    /// Close price
    pub close_price: String,
    #[serde(rename = "o")]
    /// Open price
    pub open_price: String,
    #[serde(rename = "h")]
    /// High price
    pub high_price: String,
    #[serde(rename = "l")]
    /// Low price
    pub low_price: String,
    #[serde(rename = "v")]
    /// Total traded base asset volume
    pub total_traded_base_asset_volume: String,
    #[serde(rename = "q")]
    /// Total traded quote asset volume
    pub total_traded_quote_asset_volume: String,
}
