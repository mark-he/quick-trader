use bybit_connector::ureq::{Error, Response};
use common::error::AppError;
use serde::{de::DeserializeOwned, Deserialize, Deserializer, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize,)]
pub struct BbMarketConfig {
    pub depth_level: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize,)]
pub struct Ignore {

}


pub fn get_resp_result<T: DeserializeOwned>(ret: Result<Response, Box<Error>>, skipped_code: Vec<i64>, ignore_result: bool) -> Result<Option<T>, AppError> {
    let err;
    match ret {
        Ok(resp) => {
            let ret2 = resp.into_body_str();
            match ret2 {
                Ok(data) => {
                    let mut json_value: Value = serde_json::from_str(&data).unwrap();
                    if let Some(v) = json_value.get("retCode") {
                        if v.as_i64().unwrap() == 0 {
                            if ignore_result {
                                return Ok(None)
                            } else {
                                let result = json_value.get_mut("result");
                                let obj = serde_json::from_value::<T>(result.unwrap().take()).map_err(|e| AppError::new(-200, &e.to_string()))?;
                                return Ok(Some(obj))
                            }
                        } else {
                            if skipped_code.contains(&v.as_i64().unwrap()) {
                                return Ok(None)
                            } else {
                                return Err(AppError::new(-200, json_value.get("retMsg").unwrap().as_str().unwrap()));
                            }
                        }
                    } else {
                        return Err(AppError::new(-200, "Incorrect response structure"));
                    }
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
    Err(AppError::new(-200, format!("{:?}", err).as_str()))
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BybitOrderbook {
    #[serde(rename = "topic")]
    pub topic_name: String,
    #[serde(rename = "type")]
    pub data_type: String,
    #[serde(rename = "ts")]
    pub timestamp: i64,
    #[serde(rename = "data")]
    pub data: OrderbookDataDetails,
    #[serde(rename = "cts")]
    pub creation_timestamp: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderbookDataDetails {
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "b", deserialize_with = "parse_vec_f64")]
    pub bids: Vec<Vec<f64>>,
    #[serde(rename = "a", deserialize_with = "parse_vec_f64")]
    pub asks: Vec<Vec<f64>>,
    #[serde(rename = "u")]
    pub update_id: u64,
    #[serde(rename = "seq")]
    pub sequence_number: u64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BybitKline {
    pub topic: String,
    #[serde(rename = "type")]
    pub data_type: String,
    #[serde(rename = "ts")]
    pub timestamp: i64,
    pub data: Vec<KlineDetail>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KlineDetail {
    pub start: i64,
    pub end: i64,
    pub interval: String,
    #[serde(deserialize_with = "string_to_f64")]
    pub open: f64,
    #[serde(deserialize_with = "string_to_f64")]
    pub close: f64,
    #[serde(deserialize_with = "string_to_f64")]
    pub high: f64,
    #[serde(deserialize_with = "string_to_f64")]
    pub low: f64,
    #[serde(deserialize_with = "string_to_f64")]
    pub volume: f64,
    #[serde(deserialize_with = "string_to_f64")]
    pub turnover: f64,
    pub confirm: bool,
    pub timestamp: i64,
}



#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BybitDeltaTicker {
    pub topic: String,
    #[serde(rename = "type")]
    pub data_type: String,
    pub data: DeltaTickerDetail,
    #[serde(rename = "cs")]
    pub matching_version: u64,
    #[serde(rename = "ts")]
    pub timestamp: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeltaTickerDetail {
    pub symbol: String,
    #[serde(deserialize_with = "string_to_f64")]
    pub mark_price: f64,
    #[serde(deserialize_with = "string_to_f64")]
    pub index_price: f64,
    #[serde(deserialize_with = "string_to_f64")]
    pub open_interest_value: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BybitTicker {
    pub topic: String,
    #[serde(rename = "type")]
    pub data_type: String,
    pub data: TickerDetail,
    #[serde(rename = "cs")]
    pub matching_version: u64,
    #[serde(rename = "ts")]
    pub timestamp: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TickerDetail {
    pub symbol: String,
    pub tick_direction: String,
    #[serde(deserialize_with = "string_to_f64")]
    pub price24h_pcnt: f64,
    #[serde(deserialize_with = "string_to_f64")]
    pub last_price: f64,
    #[serde(deserialize_with = "string_to_f64")]
    pub prev_price24h: f64,
    #[serde(deserialize_with = "string_to_f64")]
    pub high_price24h: f64,
    #[serde(deserialize_with = "string_to_f64")]
    pub low_price24h: f64,
    #[serde(deserialize_with = "string_to_f64")]
    pub prev_price1h: f64,
    #[serde(deserialize_with = "string_to_f64")]
    pub mark_price: f64,
    #[serde(deserialize_with = "string_to_f64")]
    pub index_price: f64,
    #[serde(deserialize_with = "string_to_f64")]
    pub open_interest: f64,
    #[serde(deserialize_with = "string_to_f64")]
    pub open_interest_value: f64,
    #[serde(deserialize_with = "string_to_f64")]
    pub turnover24h: f64,
    #[serde(deserialize_with = "string_to_f64")]
    pub volume24h: f64,
    #[serde(deserialize_with = "string_to_f64")]
    pub next_funding_time: f64,
    #[serde(deserialize_with = "string_to_f64")]
    pub funding_rate: f64,
    #[serde(deserialize_with = "string_to_f64")]
    pub bid1_price: f64,
    #[serde(deserialize_with = "string_to_f64")]
    pub bid1_size: f64,
    #[serde(deserialize_with = "string_to_f64")]
    pub ask1_price: f64,
    #[serde(deserialize_with = "string_to_f64")]
    pub ask1_size: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionData {
    pub id: String,
    pub topic: String,
    pub creation_time: i64,
    pub data: Vec<PositionDetail>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionDetail {
    pub position_idx: usize,
    pub trade_mode: usize,
    pub risk_id: usize,
    pub risk_limit_value: String,
    pub symbol: String,
    pub side: String,
    #[serde(deserialize_with = "string_to_f64")]
    pub size: f64,
    #[serde(deserialize_with = "string_to_f64")]
    pub entry_price: f64,
    pub leverage: String,
    pub position_value: String,
    pub position_balance: String,
    pub mark_price: String,
    #[serde(rename = "positionIM")]
    pub position_im: String,
    #[serde(rename = "positionMM")]
    pub position_mm: String,
    pub take_profit: String,
    pub stop_loss: String,
    pub trailing_stop: String,
    pub session_avg_price: String,
    pub unrealised_pnl: String,
    pub cur_realised_pnl: String,
    pub cum_realised_pnl: String,
    pub created_time: String,
    pub updated_time: String,
    pub tpsl_mode: String,
    pub liq_price: String,
    pub bust_price: String,
    pub category: String,
    pub position_status: String,
    pub adl_rank_indicator: usize,
    pub auto_add_margin: usize,
    pub leverage_sys_updated_time: String,
    pub mmr_sys_updated_time: String,
    pub seq: usize,
    pub is_reduce_only: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderData {
    pub id: String,
    pub topic: String,
    pub creation_time: i64,
    pub data: Vec<OrderDetail>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderDetail {
    pub symbol: String,
    pub order_id: String,
    pub side: String,
    pub order_type: String,
    pub cancel_type: String,
    #[serde(deserialize_with = "string_to_f64")]
    pub price: f64,
    #[serde(deserialize_with = "string_to_f64")]
    pub qty: f64,
    pub order_iv: String,
    pub time_in_force: String,
    pub order_status: String,
    pub order_link_id: String,
    pub last_price_on_created: String,
    pub reduce_only: bool,
    pub leaves_qty: String,
    pub leaves_value: String,
    #[serde(deserialize_with = "string_to_f64")]
    pub cum_exec_qty: f64,
    pub cum_exec_value: String,
    pub closed_pnl: String,
    pub avg_price: String,
    pub block_trade_id: String,
    pub position_idx: usize,
    pub cum_exec_fee: String,
    #[serde(deserialize_with = "string_to_f64")]
    pub created_time: f64,
    #[serde(deserialize_with = "string_to_f64")]
    pub updated_time: f64,
    pub reject_reason: String,
    pub stop_order_type: String,
    pub tpsl_mode: String,
    pub trigger_price: String,
    pub take_profit: String,
    pub stop_loss: String,
    pub tp_trigger_by: String,
    pub sl_trigger_by: String,
    pub tp_limit_price: String,
    pub sl_limit_price: String,
    pub trigger_direction: usize,
    pub trigger_by: String,
    pub close_on_trigger: bool,
    pub category: String,
    pub place_type: String,
    pub smp_type: String,
    pub smp_group: usize,
    pub smp_order_id: String,
    pub fee_currency: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletData {
    pub id: String,
    pub topic: String,
    pub creation_time: i64,
    pub data: Vec<WalletDetail>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletDetail {
    #[serde(rename = "accountIMRate")]
    pub account_im_rate: String,
    #[serde(rename = "accountMMRate")]
    pub account_mm_rate: String,
    pub total_equity: String,
    pub total_wallet_balance: String,
    pub total_margin_balance: String,
    pub total_available_balance: String,
    #[serde(rename = "totalPerpUPL")]
    pub total_perp_upl: String,
    pub total_initial_margin: String,
    pub total_maintenance_margin: String,
    pub coin: Vec<CoinDetail>,
    #[serde(rename = "accountLTV")]
    pub account_ltv: String,
    pub account_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoinDetail {
    pub coin: String,
    pub equity: String,
    pub usd_value: String,
    #[serde(deserialize_with = "string_to_f64")]
    pub wallet_balance: f64,
    #[serde(deserialize_with = "string_to_f64")]
    pub available_to_withdraw: f64,
    #[serde(deserialize_with = "string_to_f64")]
    pub available_to_borrow: f64,
    #[serde(deserialize_with = "string_to_f64")]
    pub borrow_amount: f64,
    pub accrued_interest: String,
    #[serde(rename = "totalOrderIM")]
    pub total_order_im: String,
    #[serde(rename = "totalPositionIM")]
    pub total_position_im: String,
    #[serde(rename = "totalPositionMM")]
    pub total_position_mm: String,
    #[serde(deserialize_with = "string_to_f64")]
    pub unrealised_pnl: f64,
    #[serde(deserialize_with = "string_to_f64")]
    pub cum_realised_pnl: f64,
    pub bonus: String,
    pub collateral_switch: bool,
    pub margin_collateral: bool,
    pub locked: String,
    pub spot_hedging_qty: String,
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerTime {
    #[serde(deserialize_with = "string_to_f64")]
    pub time_second: f64,
    #[serde(deserialize_with = "string_to_f64")]
    pub time_nano: f64,
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

#[derive(Debug, Clone, Serialize, Deserialize,)]
pub struct BbTradeConfig {
    pub api_key: String, 
    pub api_secret: String,
    pub settle_coin: String,
    pub position_side: u32,
    pub margin_mode: String,
}

#[derive(Debug, Clone, Serialize, Deserialize,)]
pub struct SymbolConfig {
    pub margin_type: usize, 
    pub leverage: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SymbolInfo {
    pub symbol: String, 
    pub leverage: i32,
    pub margin_type: String,
    pub dual_position_side: String,
}

fn string_to_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
   where
       D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    if s == "" {
        Ok(0 as f64)
    } else  {
        s.parse::<f64>().map_err(serde::de::Error::custom)
    }
}



#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KlineQueryResp {
    pub category: String,
    pub symbol: String,
    pub list: Vec<Vec<String>>,
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountQueryResp {
    pub list: Vec<AccountInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountInfo {
    pub account_type: String,
    #[serde(rename = "accountLTV")]
    pub account_ltv: String,
    #[serde(rename = "accountIMRate")]
    pub account_im_rate: String,
    #[serde(rename = "accountMMRate")]
    pub account_mm_rate: String,
    pub total_equity: String,
    pub total_wallet_balance: String,
    pub total_margin_balance: String,
    pub total_available_balance: String,
    #[serde(rename = "totalPerpUPL")]
    pub total_perp_upl: String,
    pub total_initial_margin: String,
    pub total_maintenance_margin: String,
    pub coin: Vec<CoinInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoinInfo {
    pub coin: String,
    pub equity: String,
    pub usd_value: String,
    #[serde(deserialize_with = "string_to_f64")]
    pub wallet_balance: f64,
    // pub free: String,
    pub locked: String,
    pub spot_hedging_qty: String,
    pub borrow_amount: String,
    #[serde(deserialize_with = "string_to_f64")]
    pub available_to_withdraw: f64,
    pub accrued_interest: String,
    #[serde(rename = "totalOrderIM")]
    pub total_order_im: String,
    #[serde(rename = "totalPositionIM")]
    pub total_position_im: String,
    #[serde(rename = "totalPositionMM")]
    pub total_position_mm: String,
    pub unrealised_pnl: String,
    pub cum_realised_pnl: String,
    pub bonus: String,
    pub margin_collateral: bool,
    pub collateral_switch: bool,
    pub available_to_borrow: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionQueryResp {
    pub category: String,
    pub list: Vec<PositionInfo>,
    pub next_page_cursor: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionInfo {
    pub position_idx: usize,
    pub risk_id: usize,
    pub risk_limit_value: String,
    pub symbol: String,
    pub side: String,
    #[serde(deserialize_with = "string_to_f64")]
    pub size: f64,
    #[serde(deserialize_with = "string_to_f64")]
    pub avg_price: f64,
    pub position_value: String,
    pub trade_mode: usize,
    pub auto_add_margin: usize,
    pub position_status: String,
    pub leverage: String,
    pub mark_price: String,
    pub liq_price: String,
    pub bust_price: String,
    #[serde(rename = "positionIM")]
    pub position_im: String,
    #[serde(rename = "positionMM")]
    pub position_mm: String,
    #[serde(deserialize_with = "string_to_f64")]
    pub position_balance: f64,
    pub take_profit: String,
    pub stop_loss: String,
    pub trailing_stop: String,
    pub session_avg_price: String,
    //pub delta: String,
    //pub gamma: String,
    //pub vega: String,
    //pub theta: String,
    pub unrealised_pnl: String,
    pub cur_realised_pnl: String,
    pub cum_realised_pnl: String,
    pub adl_rank_indicator: usize,
    pub created_time: String,
    pub updated_time: String,
    pub seq: i64,
    pub is_reduce_only: bool,
    pub mmr_sys_updated_time: String,
    pub leverage_sys_updated_time: String,
}