


use std::os::raw::*;
use std::ffi::CString;
use std::thread;
use binance::bn_market_server::BnMarketServer;
use binance::bn_trade_server::{BnTradeServer, Config};
use common::c::*;
use market::market_server::MarketData;
use crate::context;
use crate::c_model::*;


#[no_mangle]
pub extern "C" fn start() {
    let market_gateway_ref = context::get_market_gateway();
    let mut market_gateway = market_gateway_ref.lock().unwrap();
    let trade_gateway_ref = context::get_trade_gateway();
    let mut trade_gateway = trade_gateway_ref.lock().unwrap();
    let _ = market_gateway.start();
    let _ = trade_gateway.start();
}

#[no_mangle]
pub extern "C" fn subscribe_kline(sub_id : *const c_char, symbol : *const c_char, interval : *const c_char, callback: extern "C" fn(*const c_char, *const CKLine)) {
    let symbol_rust = c_char_to_string(symbol);
    let interval_rust = c_char_to_string(interval);

    let gateway_ref = context::get_market_gateway();
    let mut gateway = gateway_ref.lock().unwrap();

    let rx  = gateway.subscribe_kline(&symbol_rust, interval_rust.as_str());
    let sub_id_rust = CString::new(c_char_to_string(sub_id)).expect("CString failed");

    thread::spawn(move || {
        loop {
            if let Ok(data) = rx.recv() {
                match data {
                    MarketData::Kline(tick) => {
                        let datetime = CString::new(tick.datetime).expect("CString failed");
                        let market_data: CKLine = CKLine {
                            datetime: datetime.as_ptr(),
                            open: tick.open,
                            high: tick.high,
                            low: tick.low,
                            close: tick.close,
                            volume: tick.volume,
                            turnover: tick.turnover,
                        };
                        callback(sub_id_rust.as_ptr(), &market_data);
                    },
                    _ => {},
                }
            }
        }
    });

}

#[no_mangle]
pub extern "C" fn subscribe_tick(sub_id : *const c_char, symbol : *const c_char, callback: extern "C" fn(*const c_char, *const CTick)) {
    let symbol_rust = c_char_to_string(symbol);
    let gateway_ref = context::get_market_gateway();
    let mut gateway = gateway_ref.lock().unwrap();
    let rx  = gateway.subscribe_tick(&symbol_rust);
    let sub_id_rust = CString::new(c_char_to_string(sub_id)).expect("CString failed");
    thread::spawn(move || {
        loop {
            if let Ok(data) = rx.recv() {
                match data {
                    MarketData::Tick(tick) => {
                        let datetime = CString::new(tick.datetime).expect("CString failed");
                        let symbol = CString::new(tick.symbol).expect("CString failed");
                        let trading_day = CString::new(tick.trading_day).expect("CString failed");
                        let market_data: CTick = CTick {
                            symbol: symbol.as_ptr(),
                            trading_day: trading_day.as_ptr(),
                            datetime: datetime.as_ptr(),
                            open: tick.open,
                            high: tick.high,
                            low: tick.low,
                            close: tick.close,
                            volume: tick.volume as i32,
                            turnover: tick.turnover,
                            open_interest: tick.open_interest,
                            last_price: tick.last_price,
                            bid_price1: tick.bid_price1,
                            bid_price2: tick.bid_price2,
                            bid_price3: tick.bid_price3,
                            bid_price4: tick.bid_price4,
                            bid_price5: tick.bid_price5,
                            bid_volume1: tick.bid_volume1 as i32,
                            bid_volume2: tick.bid_volume2 as i32,
                            bid_volume3: tick.bid_volume3 as i32,
                            bid_volume4: tick.bid_volume4 as i32,
                            bid_volume5: tick.bid_volume5 as i32,
                            ask_price1: tick.ask_price1,
                            ask_price2: tick.ask_price2,
                            ask_price3: tick.ask_price3,
                            ask_price4: tick.ask_price4,
                            ask_price5: tick.ask_price5,
                            ask_volume1: tick.ask_volume1 as i32,
                            ask_volume2: tick.ask_volume2 as i32,
                            ask_volume3: tick.ask_volume3 as i32,
                            ask_volume4: tick.ask_volume4 as i32,
                            ask_volume5: tick.ask_volume5 as i32,
                        };
                        callback(sub_id_rust.as_ptr(), &market_data);
                    },
                    _ => {},
                }
            }
        }
    });

}

#[no_mangle]
pub extern "C" fn init(config: *const CTradeConfig) {
    let config_ref = unsafe { &*config };

    let env = c_char_to_string(config_ref.env);
    binance::enable_prod(env == "PROD");

    let trade_config = Config {
        api_key: c_char_to_string(config_ref.api_key),
        api_secret: c_char_to_string(config_ref.api_secret),
    };

    println!("start to initialize binance...");
    let market_server = BnMarketServer::new();
    let trade_server = BnTradeServer::new(trade_config);
    context::init(market_server, trade_server);

    let market_gateway_ref = context::get_market_gateway();
    let mut market_gateway = market_gateway_ref.lock().unwrap();
    let ret = market_gateway.connect();
    if ret.is_err() {
        panic!("{:?}", ret.unwrap_err());
    }

    let trade_gateway_ref = context::get_trade_gateway();
    let mut trade_gateway = trade_gateway_ref.lock().unwrap();
    let ret = trade_gateway.connect();
    if ret.is_err() {
        panic!("{:?}", ret.unwrap_err());
    }
    println!("binance initialized.");
}
