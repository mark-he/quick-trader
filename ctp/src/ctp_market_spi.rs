#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use market::market_server::{MarketData, Tick};
use libctp_sys::*;
use common::{c::*, msmc::Subscription};
pub struct Spi {
    tx: Subscription<MarketData>,
}

impl Spi {
    pub fn new(tx: Subscription<MarketData>) -> Self {
        Spi {tx}
    }
}

impl Rust_CThostFtdcMdSpi_Trait for Spi {
    fn on_front_connected(&mut self) {
        println!("connected.");
        self.tx.send(&Some(MarketData::Connected));
    }

    fn on_front_disconnected(&mut self, _nReason: ::std::os::raw::c_int) {
        println!("front_disconnected");
    }

    fn on_heart_beat_warning(&mut self, _nTimeLapse: ::std::os::raw::c_int) {
        println!("heart_beating");
    }

    fn on_rsp_user_login(
        &mut self,
        _pRspUserLogin: *mut CThostFtdcRspUserLoginField,
        _pRspInfo: *mut CThostFtdcRspInfoField,
        _nRequestID: ::std::os::raw::c_int,
        _bIsLast: bool,
    ) {
        println!("on_rsp_user_login");
        let pRspInfo = unsafe { &mut *_pRspInfo };
        if pRspInfo.ErrorID == 0 {
            self.tx.send(&Some(MarketData::UserLogin));
        } else {
            self.tx.send(&Some(MarketData::Error(-1,  c_char_to_gbk_string(pRspInfo.ErrorMsg.as_ptr()))));
        }
    }

    fn on_rsp_error(
        &mut self,
        _pRspInfo: *mut CThostFtdcRspInfoField,
        _nRequestID: ::std::os::raw::c_int,
        _bIsLast: bool,
    ) {
        println!("rsp_error");
    }

    fn on_rtn_depth_market_data(&mut self, pDepthMarketData: *mut CThostFtdcDepthMarketDataField) {
        if pDepthMarketData.is_null() {
            println!("got empty data");
        } else {
            let pDepthMarketData = unsafe { &mut *pDepthMarketData };
            let tick = _convert_tick(pDepthMarketData);
            self.tx.send(&Some(MarketData::Tick(tick)));
        }
    }
}

fn _convert_tick(market_data: &CThostFtdcDepthMarketDataField) -> Tick {
    let tick = Tick {
        symbol: c_char_to_string(market_data.InstrumentID.as_ptr()),
        datetime: format!("{} {}", c_char_to_string(market_data.ActionDay.as_ptr()), c_char_to_string(market_data.UpdateTime.as_ptr())),
        trading_day: c_char_to_string(market_data.TradingDay.as_ptr()),
        open: market_data.OpenPrice,
        high: market_data.HighestPrice,
        low: market_data.LowestPrice,
        close: market_data.ClosePrice,
        volume: market_data.Volume as f64,
        turnover: market_data.Turnover,
        open_interest: market_data.OpenInterest,
        last_price: market_data.LastPrice,
        bid_price1: market_data.BidPrice1,
        bid_price2: market_data.BidPrice2,
        bid_price3: market_data.BidPrice3,
        bid_price4: market_data.BidPrice4,
        bid_price5: market_data.BidPrice5,
        bid_volume1: market_data.BidVolume1 as f64,
        bid_volume2: market_data.BidVolume2 as f64,
        bid_volume3: market_data.BidVolume3 as f64,
        bid_volume4: market_data.BidVolume4 as f64,
        bid_volume5: market_data.BidVolume5 as f64,
        ask_price1: market_data.AskPrice1,
        ask_price2: market_data.AskPrice2,
        ask_price3: market_data.AskPrice3,
        ask_price4: market_data.AskPrice4,
        ask_price5: market_data.AskPrice5,
        ask_volume1: market_data.AskVolume1 as f64,
        ask_volume2: market_data.AskVolume2 as f64,
        ask_volume3: market_data.AskVolume3 as f64,
        ask_volume4: market_data.AskVolume4 as f64,
        ask_volume5: market_data.AskVolume5 as f64,
    };
    tick
}