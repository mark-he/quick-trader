use serde::{Deserialize, Serialize};

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


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrderInfo {
    #[serde(rename = "clientOrderId")]
    pub client_order_id: String,
    #[serde(rename = "cumQty")]
    pub cum_qty: String,
    #[serde(rename = "cumQuote")]
    pub cum_quote: String,
    #[serde(rename = "executedQty")]
    pub executed_qty: String,
    #[serde(rename = "orderId")]
    pub order_id: u64,
    #[serde(rename = "avgPrice")]
    pub avg_price: String,
    #[serde(rename = "origQty")]
    pub orig_qty: String,
    #[serde(rename = "price")]
    pub price: String,
    #[serde(rename = "reduceOnly")]
    pub reduce_only: bool,
    #[serde(rename = "side")]
    pub side: String,
    #[serde(rename = "positionSide")]
    pub position_side: String,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "stopPrice")]
    pub stop_price: String,
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
    #[serde(rename = "activatePrice")]
    pub activate_price: String,
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
    #[serde(rename = "wb")]
    pub wallet_balance: String,
    #[serde(rename = "cw")]
    pub cross_wallet_balance: String,
    #[serde(rename = "bc")]
    pub balance_change: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PositionData {
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "pa")]
    pub position_amount: String,
    #[serde(rename = "ep")]
    pub entry_price: String,
    #[serde(rename = "bep")]
    pub breakeven_price: String,
    #[serde(rename = "cr")]
    pub accumulated_realized: String,
    #[serde(rename = "up")]
    pub unrealized_pnl: String,
    #[serde(rename = "mt")]
    pub margin_type: String,
    #[serde(rename = "iw")]
    pub isolated_wallet: String,
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
struct OrderData {
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
    #[serde(rename = "q")]
    pub original_quantity: String,
    #[serde(rename = "p")]
    pub original_price: String,
    #[serde(rename = "ap")]
    pub average_price: String,
    #[serde(rename = "sp")]
    pub stop_price: String,
    #[serde(rename = "x")]
    pub execution_type: String,
    #[serde(rename = "X")]
    pub order_status: String,
    #[serde(rename = "i")]
    pub order_id: u64,
    #[serde(rename = "l")]
    pub order_last_filled_quantity: String,
    #[serde(rename = "z")]
    pub order_filled_accumulated_quantity: String,
    #[serde(rename = "L")]
    pub last_filled_price: String,
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
    #[serde(rename = "AP")]
    pub activation_price: String,
    #[serde(rename = "cr")]
    pub callback_rate: String,
    #[serde(rename = "pP")]
    pub is_price_protection: bool,
    #[serde(rename = "si")]
    pub ignore_si: u64,
    #[serde(rename = "ss")]
    pub ignore_ss: u64,
    #[serde(rename = "rp")]
    pub realized_profit: String,
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
    #[serde(rename = "q")]
    pub original_quantity: String,
    #[serde(rename = "p")]
    pub original_price: String,
    #[serde(rename = "m")]
    pub is_maker_side: bool,
    #[serde(rename = "c")]
    pub client_order_id: String,
    #[serde(rename = "S")]
    pub side: String,
    #[serde(rename = "L")]
    pub last_filled_price: String,
    #[serde(rename = "l")]
    pub order_last_filled_quantity: String,
    #[serde(rename = "t")]
    pub trade_id: u64,
    #[serde(rename = "i")]
    pub order_id: u64,
}

//Account info

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Asset {
    #[serde(rename = "asset")]
    pub asset_name: String,
    #[serde(rename = "walletBalance")]
    pub wallet_balance: String,
    #[serde(rename = "unrealizedProfit")]
    pub unrealized_profit: String,
    #[serde(rename = "marginBalance")]
    pub margin_balance: String,
    #[serde(rename = "maintMargin")]
    pub maint_margin: String,
    #[serde(rename = "initialMargin")]
    pub initial_margin: String,
    #[serde(rename = "positionInitialMargin")]
    pub position_initial_margin: String,
    #[serde(rename = "openOrderInitialMargin")]
    pub open_order_initial_margin: String,
    #[serde(rename = "crossWalletBalance")]
    pub cross_wallet_balance: String,
    #[serde(rename = "crossUnPnl")]
    pub cross_un_pnl: String,
    #[serde(rename = "availableBalance")]
    pub available_balance: String,
    #[serde(rename = "maxWithdrawAmount")]
    pub max_withdraw_amount: String,
    #[serde(rename = "updateTime")]
    pub update_time: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Position {
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "positionSide")]
    pub position_side: String,
    #[serde(rename = "positionAmt")]
    pub position_amt: String,
    #[serde(rename = "unrealizedProfit")]
    pub unrealized_profit: String,
    #[serde(rename = "isolatedMargin")]
    pub isolated_margin: String,
    #[serde(rename = "notional")]
    pub notional: String,
    #[serde(rename = "isolatedWallet")]
    pub isolated_wallet: String,
    #[serde(rename = "initialMargin")]
    pub initial_margin: String,
    #[serde(rename = "maintMargin")]
    pub maint_margin: String,
    #[serde(rename = "updateTime")]
    pub update_time: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountInfo {
    #[serde(rename = "totalInitialMargin")]
    pub total_initial_margin: String,
    #[serde(rename = "totalMaintMargin")]
    pub total_maint_margin: String,
    #[serde(rename = "totalWalletBalance")]
    pub total_wallet_balance: String,
    #[serde(rename = "totalUnrealizedProfit")]
    pub total_unrealized_profit: String,
    #[serde(rename = "totalMarginBalance")]
    pub total_margin_balance: String,
    #[serde(rename = "totalPositionInitialMargin")]
    pub total_position_initial_margin: String,
    #[serde(rename = "totalOpenOrderInitialMargin")]
    pub total_open_order_initial_margin: String,
    #[serde(rename = "totalCrossWalletBalance")]
    pub total_cross_wallet_balance: String,
    #[serde(rename = "totalCrossUnPnl")]
    pub total_cross_un_pnl: String,
    #[serde(rename = "availableBalance")]
    pub available_balance: String,
    #[serde(rename = "maxWithdrawAmount")]
    pub max_withdraw_amount: String,
    #[serde(rename = "assets")]
    pub assets: Vec<Asset>,
    #[serde(rename = "positions")]
    pub positions: Vec<Position>,
}