use serde::{Deserialize, Serialize};


fn string_to_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
   where
       D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    s.parse::<f64>().map_err(serde::de::Error::custom)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
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
    #[serde(rename = "o", deserialize_with = "string_to_f64")]
    pub open_price: f64,
    /// Close price
    #[serde(rename = "c", deserialize_with = "string_to_f64")]
    pub close_price: f64,
    /// High price
    #[serde(rename = "h", deserialize_with = "string_to_f64")]
    pub high_price: f64,
    /// Low price
    #[serde(rename = "l", deserialize_with = "string_to_f64")]
    pub low_price: f64,
    /// Volume of the base asset
    #[serde(rename = "v", deserialize_with = "string_to_f64")]
    pub base_asset_volume: f64,
    /// Number of trades
    #[serde(rename = "n")]
    pub number_of_trades: u64,
    /// Whether the kline is closed
    #[serde(rename = "x")]
    pub is_closed: bool,
    /// Volume of the quote asset
    #[serde(rename = "q", deserialize_with = "string_to_f64")]
    pub quote_asset_volume: f64,
    /// Taker buy volume of the base asset
    #[serde(rename = "V", deserialize_with = "string_to_f64")]
    pub taker_buy_base_asset_volume: f64,
    /// Taker buy volume of the quote asset
    #[serde(rename = "Q", deserialize_with = "string_to_f64")]
    pub taker_buy_quote_asset_volume: f64,
    /// Ignore
    #[serde(rename = "B")]
    ignored_value: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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
    #[serde(rename = "c", deserialize_with = "string_to_f64")]
    /// Close price
    pub close_price: f64,
    #[serde(rename = "o", deserialize_with = "string_to_f64")]
    /// Open price
    pub open_price: f64,
    #[serde(rename = "h", deserialize_with = "string_to_f64")]
    /// High price
    pub high_price: f64,
    #[serde(rename = "l", deserialize_with = "string_to_f64")]
    /// Low price
    pub low_price: f64,
    #[serde(rename = "v", deserialize_with = "string_to_f64")]
    /// Total traded base asset volume
    pub total_traded_base_asset_volume: f64,
    #[serde(rename = "q", deserialize_with = "string_to_f64")]
    /// Total traded quote asset volume
    pub total_traded_quote_asset_volume: f64,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrderInfo {
    #[serde(rename = "clientOrderId")]
    pub client_order_id: String,
    #[serde(rename = "cumQty", deserialize_with = "string_to_f64")]
    pub cum_qty: f64,
    #[serde(rename = "cumQuote", deserialize_with = "string_to_f64")]
    pub cum_quote: f64,
    #[serde(rename = "executedQty", deserialize_with = "string_to_f64")]
    pub executed_qty: f64,
    #[serde(rename = "orderId")]
    pub order_id: u64,
    #[serde(rename = "avgPrice", deserialize_with = "string_to_f64")]
    pub avg_price: f64,
    #[serde(rename = "origQty", deserialize_with = "string_to_f64")]
    pub orig_qty: f64,
    #[serde(rename = "price", deserialize_with = "string_to_f64")]
    pub price: f64,
    #[serde(rename = "reduceOnly")]
    pub reduce_only: bool,
    #[serde(rename = "side")]
    pub side: String,
    #[serde(rename = "positionSide")]
    pub position_side: String,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "stopPrice", deserialize_with = "string_to_f64")]
    pub stop_price: f64,
    #[serde(rename = "closePosition")]
    pub close_position: bool,
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "timeInForce")]
    pub time_in_force: String,
    #[serde(rename = "type")]
    pub order_type: String,
    #[serde(rename = "origType")]
    pub orig_type: String,
    #[serde(rename = "activatePrice", deserialize_with = "string_to_f64")]
    pub activate_price: f64,
    #[serde(rename = "priceRate")]
    pub price_rate: String,
    #[serde(rename = "updateTime")]
    pub update_time: u64,
    #[serde(rename = "workingType")]
    pub working_type: String,
    #[serde(rename = "priceProtect")]
    pub price_protect: bool,
    #[serde(rename = "priceMatch")]
    pub price_match: String,
    #[serde(rename = "selfTradePreventionMode")]
    pub self_trade_prevention_mode: String,
    #[serde(rename = "goodTillDate")]
    pub good_till_date: u64,
}

//Account update event.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BalanceData {
    #[serde(rename = "a")]
    pub asset: String,
    #[serde(rename = "wb", deserialize_with = "string_to_f64")]
    pub wallet_balance: f64,
    #[serde(rename = "cw", deserialize_with = "string_to_f64")]
    pub cross_wallet_balance: f64,
    #[serde(rename = "bc", deserialize_with = "string_to_f64")]
    pub balance_change: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PositionData {
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "pa", deserialize_with = "string_to_f64")]
    pub position_amount: f64,
    #[serde(rename = "ep", deserialize_with = "string_to_f64")]
    pub entry_price: f64,
    #[serde(rename = "bep", deserialize_with = "string_to_f64")]
    pub breakeven_price: f64,
    #[serde(rename = "cr", deserialize_with = "string_to_f64")]
    pub accumulated_realized: f64,
    #[serde(rename = "up", deserialize_with = "string_to_f64")]
    pub unrealized_pnl: f64,
    #[serde(rename = "mt")]
    pub margin_type: String,
    #[serde(rename = "iw", deserialize_with = "string_to_f64")]
    pub isolated_wallet: f64,
    #[serde(rename = "ps")]
    pub position_side: String,
    
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccountData {
    #[serde(rename = "m")]
    pub event_reason_type: String,
    #[serde(rename = "B")]
    pub balances: Vec<BalanceData>,
    #[serde(rename = "P")]
    pub positions: Vec<PositionData>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccountUpdateEvent {
    #[serde(rename = "e")]
    pub event_type: String,
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "T")]
    pub transaction: u64,
    #[serde(rename = "a")]
    pub update_data: AccountData,
}

//Order trade update event
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrderData {
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "c")]
    pub client_order_id: String,
    #[serde(rename = "S")]
    pub side: String,
    #[serde(rename = "o")]
    pub order_type: String,
    #[serde(rename = "f")]
    pub time_in_force: String,
    #[serde(rename = "q", deserialize_with = "string_to_f64")]
    pub original_quantity: f64,
    #[serde(rename = "p", deserialize_with = "string_to_f64")]
    pub original_price: f64,
    #[serde(rename = "ap", deserialize_with = "string_to_f64")]
    pub average_price: f64,
    #[serde(rename = "sp", deserialize_with = "string_to_f64")]
    pub stop_price: f64,
    #[serde(rename = "x")]
    pub execution_type: String,
    #[serde(rename = "X")]
    pub order_status: String,
    #[serde(rename = "i")]
    pub order_id: u64,
    #[serde(rename = "l", deserialize_with = "string_to_f64")]
    pub order_last_filled_quantity: f64,
    #[serde(rename = "z", deserialize_with = "string_to_f64")]
    pub order_filled_accumulated_quantity: f64,
    #[serde(rename = "L", deserialize_with = "string_to_f64")]
    pub last_filled_price: f64,
    #[serde(rename = "N")]
    pub commission_asset: String,
    #[serde(rename = "n")]
    pub commission: String,
    #[serde(rename = "T")]
    pub order_trade_time: u64,
    #[serde(rename = "t")]
    pub trade_id: u64,
    #[serde(rename = "b")]
    pub bids_notional: String,
    #[serde(rename = "a")]
    pub ask_notional: String,
    #[serde(rename = "m")]
    pub is_maker_side: bool,
    #[serde(rename = "R")]
    pub is_reduce_only: bool,
    #[serde(rename = "wt")]
    pub stop_price_working_type: String,
    #[serde(rename = "ot")]
    pub original_order_type: String,
    #[serde(rename = "ps")]
    pub position_side: String,
    #[serde(rename = "cp")]
    pub is_close_all: bool,
    //#[serde(rename = "AP")]
    //pub activation_price: String,
    //#[serde(rename = "cr")]
    //pub callback_rate: String,
    #[serde(rename = "pP")]
    pub is_price_protection: bool,
    #[serde(rename = "si")]
    pub ignore_si: u64,
    #[serde(rename = "ss")]
    pub ignore_ss: u64,
    #[serde(rename = "rp", deserialize_with = "string_to_f64")]
    pub realized_profit: f64,
    #[serde(rename = "V")]
    pub stp_mode: String,
    #[serde(rename = "pm")]
    pub price_match_mode: String,
    #[serde(rename = "gtd")]
    pub tif_gtd_order_auto_cancel_time: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrderTradeUpdateEvent {
    #[serde(rename = "e")]
    pub event_type: String,
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "T")]
    pub transaction_time: u64,
    #[serde(rename = "o")]
    pub order: OrderData,
}

//Trade lite event

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TradeLiteEvent {
    #[serde(rename = "e")]
    pub event_type: String,
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "T")]
    pub transaction_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "q", deserialize_with = "string_to_f64")]
    pub original_quantity: f64,
    #[serde(rename = "p", deserialize_with = "string_to_f64")]
    pub original_price: f64,
    #[serde(rename = "m")]
    pub is_maker_side: bool,
    #[serde(rename = "c")]
    pub client_order_id: String,
    #[serde(rename = "S")]
    pub side: String,
    #[serde(rename = "L", deserialize_with = "string_to_f64")]
    pub last_filled_price: f64,
    #[serde(rename = "l", deserialize_with = "string_to_f64")]
    pub order_last_filled_quantity: f64,
    #[serde(rename = "t")]
    pub trade_id: u64,
    #[serde(rename = "i")]
    pub order_id: u64,
}

//Account info

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Account {
    #[serde(rename = "feeTier")]
    pub fee_tier: u32,
    #[serde(rename = "feeBurn")]
    pub fee_burn: bool,
    #[serde(rename = "canTrade")]
    pub can_trade: bool,
    #[serde(rename = "canDeposit")]
    pub can_deposit: bool,
    #[serde(rename = "canWithdraw")]
    pub can_withdraw: bool,
    #[serde(rename = "updateTime")]
    pub update_time: u64,
    #[serde(rename = "multiAssetsMargin")]
    pub multi_assets_margin: bool,
    #[serde(rename = "tradeGroupId")]
    pub trade_group_id: i32,
    #[serde(rename = "totalInitialMargin", deserialize_with = "string_to_f64")]
    pub total_initial_margin: f64,
    #[serde(rename = "totalMaintMargin", deserialize_with = "string_to_f64")]
    pub total_maint_margin: f64,
    #[serde(rename = "totalWalletBalance", deserialize_with = "string_to_f64")]
    pub total_wallet_balance: f64,
    #[serde(rename = "totalUnrealizedProfit", deserialize_with = "string_to_f64")]
    pub total_unrealized_profit: f64,
    #[serde(rename = "totalMarginBalance", deserialize_with = "string_to_f64")]
    pub total_margin_balance: f64,
    #[serde(rename = "totalPositionInitialMargin", deserialize_with = "string_to_f64")]
    pub total_position_initial_margin: f64,
    #[serde(rename = "totalOpenOrderInitialMargin", deserialize_with = "string_to_f64")]
    pub total_open_order_initial_margin: f64,
    #[serde(rename = "totalCrossWalletBalance", deserialize_with = "string_to_f64")]
    pub total_cross_wallet_balance: f64,
    #[serde(rename = "totalCrossUnPnl", deserialize_with = "string_to_f64")]
    pub total_cross_unpnl: f64,
    #[serde(rename = "availableBalance", deserialize_with = "string_to_f64")]
    pub available_balance: f64,
    #[serde(rename = "maxWithdrawAmount", deserialize_with = "string_to_f64")]
    pub max_withdraw_amount: f64,
    pub assets: Vec<Asset>,
    pub positions: Vec<Position>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Asset {
    #[serde(rename = "asset")]
    pub asset: String,
    #[serde(rename = "walletBalance", deserialize_with = "string_to_f64")]
    pub wallet_balance: f64,
    #[serde(rename = "unrealizedProfit", deserialize_with = "string_to_f64")]
    pub unrealized_profit: f64,
    #[serde(rename = "marginBalance", deserialize_with = "string_to_f64")]
    pub margin_balance: f64,
    #[serde(rename = "maintMargin", deserialize_with = "string_to_f64")]
    pub maint_margin: f64,
    #[serde(rename = "initialMargin", deserialize_with = "string_to_f64")]
    pub initial_margin: f64,
    #[serde(rename = "positionInitialMargin", deserialize_with = "string_to_f64")]
    pub position_initial_margin: f64,
    #[serde(rename = "openOrderInitialMargin", deserialize_with = "string_to_f64")]
    pub open_order_initial_margin: f64,
    #[serde(rename = "crossWalletBalance", deserialize_with = "string_to_f64")]
    pub cross_wallet_balance: f64,
    #[serde(rename = "crossUnPnl", deserialize_with = "string_to_f64")]
    pub cross_unpnl: f64,
    #[serde(rename = "availableBalance", deserialize_with = "string_to_f64")]
    pub available_balance: f64,
    #[serde(rename = "maxWithdrawAmount", deserialize_with = "string_to_f64")]
    pub max_withdraw_amount: f64,
    #[serde(rename = "marginAvailable")]
    pub margin_available: bool,
    #[serde(rename = "updateTime")]
    pub update_time: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Position {
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "initialMargin", deserialize_with = "string_to_f64")]
    pub initial_margin: f64,
    #[serde(rename = "maintMargin", deserialize_with = "string_to_f64")]
    pub maint_margin: f64,
    #[serde(rename = "unrealizedProfit", deserialize_with = "string_to_f64")]
    pub unrealized_profit: f64,
    #[serde(rename = "positionInitialMargin", deserialize_with = "string_to_f64")]
    pub position_initial_margin: f64,
    #[serde(rename = "openOrderInitialMargin", deserialize_with = "string_to_f64")]
    pub open_order_initial_margin: f64,
    #[serde(rename = "leverage")]
    pub leverage: String,
    #[serde(rename = "isolated")]
    pub isolated: bool,
    #[serde(rename = "entryPrice", deserialize_with = "string_to_f64")]
    pub entry_price: f64,
    #[serde(rename = "maxNotional", deserialize_with = "string_to_f64")]
    pub max_notional: f64,
    #[serde(rename = "bidNotional", deserialize_with = "string_to_f64")]
    pub bid_notional: f64,
    #[serde(rename = "askNotional", deserialize_with = "string_to_f64")]
    pub ask_notional: f64,
    #[serde(rename = "positionSide")]
    pub position_side: String,
    #[serde(rename = "positionAmt", deserialize_with = "string_to_f64")]
    pub position_amt: f64,
    #[serde(rename = "updateTime")]
    pub update_time: u64,
}