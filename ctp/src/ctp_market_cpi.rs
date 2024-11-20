#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use market::market_server::{MarketData, Tick};
use libctp_sys::*;
use common::{c::*, msmc::Subscription};
use log::*;
use chrono::NaiveDateTime;

pub struct Spi {
    subscription: Subscription<MarketData>,
}

impl Spi {
    pub fn new(subscription: Subscription<MarketData>) -> Self {
        Spi {subscription}
    }
}

impl Rust_CThostFtdcMdSpi_Trait for Spi {
    fn on_front_connected(&mut self) {
        debug!("Cpi connected");
        self.subscription.send(&MarketData::Connected);
    }

    fn on_front_disconnected(&mut self, _nReason: ::std::os::raw::c_int) {
        debug!("Cpi disconnected");
    }

    fn on_heart_beat_warning(&mut self, _nTimeLapse: ::std::os::raw::c_int) {
        debug!("Cpi heart beat warning");
    }

    fn on_rsp_user_login(
        &mut self,
        _pRspUserLogin: *mut CThostFtdcRspUserLoginField,
        _pRspInfo: *mut CThostFtdcRspInfoField,
        _nRequestID: ::std::os::raw::c_int,
        _bIsLast: bool,
    ) {
        debug!("Cpi user login");
        let pRspInfo = unsafe { &mut *_pRspInfo };
        if pRspInfo.ErrorID == 0 {
            self.subscription.send(&MarketData::UserLogin);
        } else {
            self.subscription.send(&MarketData::Error(-1,  c_char_to_gbk_string(pRspInfo.ErrorMsg.as_ptr())));
        }
    }

    fn on_rsp_error(
        &mut self,
        _pRspInfo: *mut CThostFtdcRspInfoField,
        _nRequestID: ::std::os::raw::c_int,
        _bIsLast: bool,
    ) {
        error!("Cpi error")
    }

    fn on_rtn_depth_market_data(&mut self, pDepthMarketData: *mut CThostFtdcDepthMarketDataField) {
        if pDepthMarketData.is_null() {
            warn!("Cpi got empty data");
        } else {
            let pDepthMarketData = unsafe { &mut *pDepthMarketData };
            let tick = _convert_tick(pDepthMarketData);
            self.subscription.send(&MarketData::Tick(tick));
        }
    }
}

fn _convert_tick(market_data: &CThostFtdcDepthMarketDataField) -> Tick {
    let bids = vec![
        vec![market_data.BidPrice1, market_data.BidVolume1 as f64], 
        vec![market_data.BidPrice2, market_data.BidVolume2 as f64],
        vec![market_data.BidPrice3, market_data.BidVolume3 as f64],
        vec![market_data.BidPrice4, market_data.BidVolume4 as f64],
        vec![market_data.BidPrice5, market_data.BidVolume5 as f64],
        ];
    let asks = vec![
        vec![market_data.AskPrice1, market_data.AskVolume1 as f64], 
        vec![market_data.AskPrice2, market_data.AskVolume2 as f64],
        vec![market_data.AskPrice3, market_data.AskVolume3 as f64],
        vec![market_data.AskPrice4, market_data.AskVolume4 as f64],
        vec![market_data.AskPrice5, market_data.AskVolume5 as f64],
        ];
    let datetime = format!("{} {}", _format_date(&c_char_to_string(market_data.ActionDay.as_ptr())), c_char_to_string(market_data.UpdateTime.as_ptr()));
    let timestamp = NaiveDateTime::parse_from_str(&datetime, "%Y-%m-%d %H:%M:%S").unwrap().and_utc().timestamp() as u64 * 1000;

    let tick = Tick {
        symbol: c_char_to_string(market_data.InstrumentID.as_ptr()),
        datetime: datetime,
        trading_day: _format_date(&c_char_to_string(market_data.TradingDay.as_ptr())),
        open: market_data.OpenPrice,
        high: market_data.HighestPrice,
        low: market_data.LowestPrice,
        close: market_data.LastPrice,
        volume: market_data.Volume as f64,
        turnover: market_data.Turnover,
        timestamp: timestamp,
        bids: bids,
        asks: asks,
    };
    tick
}

fn _format_date(date_str: &str) -> String {
    let year = &date_str[0..4];
    let month = &date_str[4..6];
    let day = &date_str[6..8];
    format!("{}-{}-{}", year, month, day)
}
