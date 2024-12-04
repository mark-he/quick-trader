use std::os::raw::*;
use std::ffi::CString;
use std::thread;
use common::c::*;
use market::market_server::{KLine, MarketData};
use serde_json::Value;
use trade::trade_server::{Position, TradeEvent, Wallet};
use crate::c_model::ServiceResult;
use crate::context;
use log::*;


#[no_mangle]
pub extern "C" fn init(exchange: *const c_char, mode: *const c_char, config: *const c_char) -> Box<CString> {
    let mut result = ServiceResult::<String>::new(0, "", None);

    let exchange_rust = c_char_to_string(exchange);
    let mode_rust = c_char_to_string(mode);
    let config_rust = c_char_to_string(config);

    let ret = context::init(&exchange_rust, &mode_rust, &config_rust);
    if ret.is_err() {
        result.error_code = -1;
        result.message = format!("{:?}", ret.unwrap_err());
    } 
    if result.error_code == 0 {
        let market_gateway_ref = context::get_market_gateway();
        let mut market_gateway = market_gateway_ref.lock().unwrap();
        let ret = market_gateway.init();
        if ret.is_err() {
            result.error_code = -1;
            result.message = format!("{:?}", &ret.unwrap_err());
        }
    }
    if result.error_code == 0 {
        let trade_gateway_ref = context::get_trade_gateway();
        let mut trade_gateway = trade_gateway_ref.lock().unwrap();
        let ret = trade_gateway.init();

        if ret.is_err() {
            result.error_code = -1;
            result.message = format!("{:?}", ret.unwrap_err());
        }
    }
    result.to_c_json()
}

#[no_mangle]
pub extern "C" fn close() -> Box<CString> {
    let result = ServiceResult::<usize>::new(0, "", None);
    let market_gateway_ref = context::get_market_gateway();
    let market_gateway = market_gateway_ref.lock().unwrap();
    let _ = market_gateway.close();

    let trade_gateway_ref = context::get_trade_gateway();
    let trade_gateway = trade_gateway_ref.lock().unwrap();
    let _ = trade_gateway.close();
    debug!("Engine closed!");
    result.to_c_json()
}

#[no_mangle]
pub extern "C" fn get_server_ping() -> Box<CString> {
    let mut result = ServiceResult::<usize>::new(0, "", None);

    let market_gateway_ref = context::get_market_gateway();
    let market_gateway = market_gateway_ref.lock().unwrap();

    let server_ping = market_gateway.get_server_ping();
    if server_ping > 0 {
        result.data = Some(server_ping);
    }
    result.to_c_json()
}

#[no_mangle]
pub extern "C" fn start() -> Box<CString> {
    let mut result = ServiceResult::<String>::new(0, "", None);

    let market_gateway_ref = context::get_market_gateway();
    let mut market_gateway = market_gateway_ref.lock().unwrap();
    let ret = market_gateway.start();
    if ret.is_err() {
        result.error_code = -1;
        result.message = format!("{:?}", ret.unwrap_err());
    } 
    if result.error_code == 0 {
        let trade_gateway_ref = context::get_trade_gateway();
        let mut trade_gateway = trade_gateway_ref.lock().unwrap();
        let ret = trade_gateway.start();
        if ret.is_err() {
            result.error_code = -1;
            result.message = format!("{:?}", ret.unwrap_err());
        }
    }
    result.to_c_json()
}

#[no_mangle]
pub extern "C" fn subscribe_kline(sub_id : *const c_char, symbol : *const c_char, interval : *const c_char, count: i32, callback: extern "C" fn(*const c_char, *const c_char)) -> Box<CString> {
    let mut result = ServiceResult::<Vec<KLine>>::new(0, "", None);

    let symbol_rust = c_char_to_string(symbol);
    let interval_rust = c_char_to_string(interval);

    let gateway_ref = context::get_market_gateway();
    let mut gateway = gateway_ref.lock().unwrap();

    let rx  = gateway.subscribe_kline(symbol_rust.clone(), interval_rust.as_str());
    let sub_id_rust = CString::new(c_char_to_string(sub_id)).expect("CString failed");
    thread::spawn(move || {
        loop {
            if let Ok(data) = rx.recv() {
                match data {
                    MarketData::Kline(k) => {
                        let json = serde_json::to_string(&k).unwrap();
                        let json_rust = CString::new(json).expect("CString failed");
                        callback(sub_id_rust.as_ptr(), json_rust.as_ptr());
                    },
                    _ => {},
                }
            }
        }
    });
    let ret = gateway.load_kline(symbol_rust, &interval_rust, count as u32);
    match ret {
        Ok(klines) => {
            result.data = Some(klines);
        },
        Err(e) => {
            result.error_code = -1;
            result.message = format!("{:?}", e);
        }
    }
    result.to_c_json()
}

#[no_mangle]
pub extern "C" fn subscribe_tick(sub_id : *const c_char, symbol : *const c_char, callback: extern "C" fn(*const c_char, *const c_char)) -> Box<CString> {
    let result = ServiceResult::<String>::new(0, "", None);

    let symbol_rust = c_char_to_string(symbol);
    let gateway_ref = context::get_market_gateway();
    let mut gateway = gateway_ref.lock().unwrap();
    let rx  = gateway.subscribe_tick(symbol_rust);
    let sub_id_rust = CString::new(c_char_to_string(sub_id)).expect("CString failed");
    thread::spawn(move || {
        loop {
            if let Ok(data) = rx.recv() {
                match data {
                    MarketData::Tick(tick) => {
                        let json = serde_json::to_string(&tick).unwrap();
                        let json_rust = CString::new(json).expect("CString failed");
                        callback(sub_id_rust.as_ptr(), json_rust.as_ptr());
                    },
                    _ => {},
                }
            }
        }
    });
    result.to_c_json()
}

#[no_mangle]
pub extern "C" fn new_order(symbol : *const c_char, order_request: *const c_char) -> Box<CString> {
    let mut result = ServiceResult::<String>::new(0, "", None);
    let symbol_rust = c_char_to_string(symbol);
    let order_request_rust = c_char_to_string(order_request);

    let gateway_ref = context::get_trade_gateway();
    let mut gateway = gateway_ref.lock().unwrap();
    let ret = gateway.new_order(symbol_rust, &order_request_rust);
    if ret.is_err() {
        result.error_code = -1;
        result.message = format!("{:?}", ret.unwrap_err());
    }
    result.to_c_json()
}

#[no_mangle]
pub extern "C" fn cancel_order(symbol : *const c_char, order_id : *const c_char) -> Box<CString> {
    let mut result = ServiceResult::<String>::new(0, "", None);
    let symbol_rust = c_char_to_string(symbol);
    let order_id_rust = c_char_to_string(order_id);

    let gateway_ref = context::get_trade_gateway();
    let mut gateway = gateway_ref.lock().unwrap();

    let ret = gateway.cancel_order(symbol_rust, &order_id_rust);
    if ret.is_err() {
        result.error_code = -1;
        result.message = format!("{:?}", ret.unwrap_err());
    }
    result.to_c_json()
}

#[no_mangle]
pub extern "C" fn cancel_orders(symbol : *const c_char) -> Box<CString> {
    let mut result = ServiceResult::<String>::new(0, "", None);
    let symbol_rust = c_char_to_string(symbol);

    let gateway_ref = context::get_trade_gateway();
    let mut gateway = gateway_ref.lock().unwrap();

    let ret = gateway.cancel_orders(symbol_rust);
    if ret.is_err() {
        result.error_code = -1;
        result.message = format!("{:?}", ret.unwrap_err());
    }
    result.to_c_json()
}

#[no_mangle]
pub extern "C" fn get_positions(symbol : *const c_char) -> Box<CString> {
    let mut result = ServiceResult::<Vec<Position>>::new(0, "", None);
    let symbol_rust = c_char_to_string(symbol);
    let gateway_ref = context::get_trade_gateway();
    let mut gateway = gateway_ref.lock().unwrap();

    
    let ret = gateway.get_positions(symbol_rust);
    if ret.is_err() {
        result.error_code = -1;
        result.message = format!("{:?}", ret.unwrap_err());
    } else {
        result.data = Some(ret.unwrap());
    }
    result.to_c_json()
}

#[no_mangle]
pub extern "C" fn get_account(asset : *const c_char) -> Box<CString> {
    let mut result = ServiceResult::<Option<Wallet>>::new(0, "", None);
    let asset_rust = c_char_to_string(asset);
    let gateway_ref = context::get_trade_gateway();
    let mut gateway = gateway_ref.lock().unwrap();
    let ret = gateway.get_account(&asset_rust);
    if ret.is_err() {
        result.error_code = -1;
        result.message = format!("{:?}", ret.unwrap_err());
    } else {
        result.data = Some(ret.unwrap());
    }
    result.to_c_json()
}

#[no_mangle]
pub extern "C" fn init_symbol_trade(sub_id: *const c_char, symbol: *const c_char, config: *const c_char, callback: extern "C" fn(*const c_char, *const c_char, *const c_char)) -> Box<CString> {
    let mut result = ServiceResult::<Value>::new(0, "", None);

    let gateway_ref = context::get_trade_gateway();
    let mut gateway = gateway_ref.lock().unwrap();

    let symbol_rust = c_char_to_string(symbol);
    let config_rust = c_char_to_string(config);

    let ret = gateway.init_symbol(symbol_rust.clone(), &config_rust);
    match ret {
        Ok(symbol_info) => {
            result.data = Some(symbol_info);
        },
        Err(e) => {
            result.error_code = -1;
            result.message = format!("{:?}", e);
        },
    }
    if result.error_code == 0 {
        let rx = gateway.register_symbol(symbol_rust.clone());
        let sub_id_rust = CString::new(c_char_to_string(sub_id)).expect("CString failed");
        thread::spawn(move || {
            loop {
                if let Ok(data) = rx.recv() {
                    match data {
                        TradeEvent::OrderUpdate(order) => {
                            if symbol_rust == order.symbol {
                                let json = serde_json::to_string(&order).unwrap();
                                let json_rust = CString::new(json).expect("CString failed");
                                let _type = CString::new("ORDER".to_string()).expect("CString failed");
                                callback(sub_id_rust.as_ptr(), _type.as_ptr(), json_rust.as_ptr());
                            }
                        },
                        TradeEvent::PositionUpdate(position) => {
                            let json = serde_json::to_string(&position).unwrap();
                            let json_rust = CString::new(json).expect("CString failed");
                            let _type = CString::new("POSITION".to_string()).expect("CString failed");
                            callback(sub_id_rust.as_ptr(), _type.as_ptr(), json_rust.as_ptr());
                        }
                        TradeEvent::AccountUpdate(wallet) => {
                            let json = serde_json::to_string(&wallet).unwrap();
                            let json_rust = CString::new(json).expect("CString failed");
                            let _type = CString::new("ACCOUNT".to_string()).expect("CString failed");
                            callback(sub_id_rust.as_ptr(), _type.as_ptr(), json_rust.as_ptr());
                        },
                    }
                }
            }
        });
    }
    result.to_c_json()
}
