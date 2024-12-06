use binance_future_connector::{http::error::ClientError, market_stream::enums::{Level, UpdateSpeed}, trade::enums::{MarginAssetMode, MarginType, PositionMode}, ureq::{Error, Response}};
use common::error::AppError;
use serde::{Deserialize, Deserializer, Serialize};

pub fn get_resp_result(ret: Result<Response, Box<Error>>, skipped_code: Vec<i16>) -> Result<String, AppError> {
    let err;
    match ret {
        Ok(resp) => {
            let ret2 = resp.into_body_str();
            match ret2 {
                Ok(data) => {
                    return Ok(data);
                },
                Err(e) => {
                    err = *e;
                },
            }
        },
        Err(e) => {
            err = *e;
        },
    }
    match err {
        Error::Client(ClientError::Structured(http)) => {
            if skipped_code.contains(&http.data.code) {
                Ok("".to_string())
            } else {
                Err(AppError::new(-200, format!("{:?}", &http.data.message).as_str()))
            }
        },
        _ => {
            Err(AppError::new(-200, format!("{:?}", err).as_str()))
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize,)]
pub struct SymbolConfig {
    pub margin_type: MarginType, 
    pub leverage: i32,
}

impl SymbolConfig {
    pub fn new() -> Self {
        SymbolConfig {
            margin_type: MarginType::Isolated,
            leverage: 5,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SymbolInfo {
    pub symbol: String, 
    pub leverage: i32,
    pub margin_type: MarginType,
    pub dual_position_side: PositionMode,
    pub multi_assets_margin: MarginAssetMode,
    pub maint_margin_ratio: f64,
    pub quantity_precision: usize,
    pub price_precision: usize,
    pub quote_precision: usize,
}


#[derive(Debug, Clone, Serialize, Deserialize,)]
pub struct BnMarketConfig {
    pub tick_update_speed: Option<UpdateSpeed>,
    pub depth_level: Level,
}


#[derive(Debug, Clone, Serialize, Deserialize,)]
pub struct BnTradeConfig {
    pub api_key: String, 
    pub api_secret: String,
    pub dual_position_side: PositionMode,
    pub multi_assets_margin: MarginAssetMode,
}


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
    pub volume: f64,
    /// Number of trades
    #[serde(rename = "n")]
    pub number_of_trades: u64,
    /// Whether the kline is closed
    #[serde(rename = "x")]
    pub is_closed: bool,
    /// Volume of the quote asset
    #[serde(rename = "q", deserialize_with = "string_to_f64")]
    pub turnover: f64,
    /// Taker buy volume of the base asset
    #[serde(rename = "V", deserialize_with = "string_to_f64")]
    pub taker_buy_volume: f64,
    /// Taker buy volume of the quote asset
    #[serde(rename = "Q", deserialize_with = "string_to_f64")]
    pub taker_buy_turnover: f64,
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
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
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
pub struct AccountQueryResp {
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
    pub assets: Vec<AssetResp>,
    pub positions: Vec<PositionResp>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct AssetResp {
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
pub struct PositionResp {
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


#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BracketResp {
    #[serde(rename = "bracket")]
    pub bracket: usize,
    #[serde(rename = "initialLeverage")]
    pub initial_leverge: usize,
    #[serde(rename = "notionalCap")]
    pub notional_cap: usize,
    #[serde(rename = "notionalFloor")]
    pub notional_floor: usize,
    #[serde(rename = "maintMarginRatio")]
    pub maint_margin_ratio: f64,
    #[serde(rename = "cum")]
    pub cum: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct LeverageBracketQueryResp {
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "brackets")]
    pub brackets: Vec<BracketResp>,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RateLimit {
    #[serde(rename = "interval")]
    pub interval: String,
    #[serde(rename = "intervalNum")]
    pub interval_num: usize,
    #[serde(rename = "limit")]
    pub limit: usize,
    #[serde(rename = "rateLimitType")]
    pub rate_limit_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AssetInfo {
    #[serde(rename = "asset")]
    pub asset: String,
    #[serde(rename = "marginAvailable")]
    pub margin_available: bool,
    #[serde(rename = "autoAssetExchange")]
    pub auto_asset_exchange: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Filter {
    #[serde(rename = "filterType")]
    pub filter_type: String,
    #[serde(rename = "maxPrice")]
    pub max_price: Option<String>,
    #[serde(rename = "minPrice")]
    pub min_price: Option<String>,
    #[serde(rename = "tickSize")]
    pub tick_size: Option<String>,
    #[serde(rename = "maxQty")]
    pub max_qty: Option<String>,
    #[serde(rename = "minQty")]
    pub min_qty: Option<String>,
    #[serde(rename = "stepSize")]
    pub step_size: Option<String>,
    #[serde(rename = "limit")]
    pub limit: Option<usize>,
    #[serde(rename = "notional")]
    pub notional: Option<String>,
    #[serde(rename = "multiplierUp")]
    pub multiplier_up: Option<String>,
    #[serde(rename = "multiplierDown")]
    pub multiplier_down: Option<String>,
    #[serde(rename = "multiplierDecimal")]
    pub multiplier_decimal: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Contract {
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "pair")]
    pub pair: String,
    #[serde(rename = "contractType")]
    pub contract_type: String,
    #[serde(rename = "deliveryDate")]
    pub delivery_date: usize,
    #[serde(rename = "onboardDate")]
    pub onboard_date: usize,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "maintMarginPercent")]
    pub maint_margin_percent: String,
    #[serde(rename = "requiredMarginPercent")]
    pub required_margin_percent: String,
    #[serde(rename = "baseAsset")]
    pub base_asset: String,
    #[serde(rename = "quoteAsset")]
    pub quote_asset: String,
    #[serde(rename = "marginAsset")]
    pub margin_asset: String,
    #[serde(rename = "pricePrecision")]
    pub price_precision: usize,
    #[serde(rename = "quantityPrecision")]
    pub quantity_precision: usize,
    #[serde(rename = "baseAssetPrecision")]
    pub base_asset_precision: usize,
    #[serde(rename = "quotePrecision")]
    pub quote_precision: usize,
    #[serde(rename = "underlyingType")]
    pub underlying_type: String,
    #[serde(rename = "underlyingSubType")]
    pub underlying_sub_type: Vec<String>,
    //#[serde(rename = "settlePlan")]
    //pub settle_plan: usize,
    #[serde(rename = "triggerProtect")]
    pub trigger_protect: String,
    #[serde(rename = "filters")]
    pub filters: Vec<Filter>,
    #[serde(rename = "orderTypes")]
    pub order_types: Vec<String>,
    #[serde(rename = "timeInForce")]
    pub time_in_force: Vec<String>,
    #[serde(rename = "liquidationFee")]
    pub liquidation_fee: String,
    #[serde(rename = "marketTakeBound")]
    pub market_take_bound: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExchangeInfoQueryResp {
    #[serde(rename = "exchangeFilters")]
    pub exchange_filters: Vec<String>,
    #[serde(rename = "rateLimits")]
    pub rate_limits: Vec<RateLimit>,
    #[serde(rename = "serverTime")]
    pub server_time: usize,
    #[serde(rename = "assets")]
    pub assets: Vec<AssetInfo>,
    #[serde(rename = "symbols")]
    pub symbols: Vec<Contract>,
    #[serde(rename = "timezone")]
    pub timezone: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BinanceDepthUpdate {
    #[serde(rename = "e")]
    pub event_type: String,
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "T")]
    pub transaction_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "U")]
    pub first_update_id: u64,
    #[serde(rename = "u")]
    pub final_update_id: u64,
    #[serde(rename = "pu")]
    pub final_update_id_in_last_stream: u64,
    #[serde(rename = "b", deserialize_with = "parse_vec_f64")]
    pub bids: Vec<Vec<f64>>,
    #[serde(rename = "a", deserialize_with = "parse_vec_f64")]
    pub asks: Vec<Vec<f64>>,
}

fn parse_vec_f64<'de, D>(deserializer: D) -> Result<Vec<Vec<f64>>, D::Error>
where
    D: Deserializer<'de>,
{
    let strings: Vec<Vec<String>> = Deserialize::deserialize(deserializer)?;
    let mut parsed_f64s = Vec::new();
    for s in strings {
        let mut parsed_f64 = Vec::new();
        for value in s {
            parsed_f64.push(value.parse::<f64>().map_err(serde::de::Error::custom)?);
        }
        parsed_f64s.push(parsed_f64);
    }
    Ok(parsed_f64s)
}