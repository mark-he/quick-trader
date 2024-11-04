

use std::os::raw::*;
use std::ffi::CString;
use std::thread;
use binance::bn_market_server::BnMarketServer;
use binance::bn_trade_server::{AccountEvent, BnTradeServer, Config, SymbolConfig};
use binance_future_connector::trade::new_order::NewOrderRequest;
use chrono::DateTime;
use common::c::*;
use market::market_server::{KLine, MarketData};
use crate::c_model::OrderEvent;
use crate::context;

#[no_mangle]
pub extern "C" fn start() {
    let market_gateway_ref = context::get_market_gateway();
    let mut market_gateway = market_gateway_ref.lock().unwrap();
    let _ = market_gateway.start();
    let trade_gateway_ref = context::get_trade_gateway();
    let mut trade_gateway = trade_gateway_ref.lock().unwrap();
    let _ = trade_gateway.start();
}

#[no_mangle]
pub extern "C" fn subscribe_kline(sub_id : *const c_char, symbol : *const c_char, interval : *const c_char, count: i32, callback: extern "C" fn(*const c_char, *const c_char)) -> Box<CString> {
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
    let ret = gateway.load_kline(&symbol_rust, &interval_rust, count as u32);
    if ret.is_err() {
        panic!("{:?}", ret.unwrap_err());
    }
    let klines: Vec<KLine> = ret.unwrap();
    let json = serde_json::to_string(&klines).unwrap();
    Box::new(CString::new(json).unwrap())
}

#[no_mangle]
pub extern "C" fn subscribe_tick(sub_id : *const c_char, symbol : *const c_char, callback: extern "C" fn(*const c_char, *const c_char)) {
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
                        let json = serde_json::to_string(&tick).unwrap();
                        let json_rust = CString::new(json).expect("CString failed");
                        callback(sub_id_rust.as_ptr(), json_rust.as_ptr());
                    },
                    _ => {},
                }
            }
        }
    });
}

#[no_mangle]
pub extern "C" fn init(env: *const c_char, config: *const c_char) {
    let env_rust = c_char_to_string(env);
    let config_rust = c_char_to_string(config);
    let ret = serde_json::from_str::<Config>(&config_rust);
    if ret.is_err() {
        panic!("{:?}", ret.unwrap_err());
    }

    binance::enable_prod(env_rust == "PROD");
    println!("start to initialize binance...");
    let market_server = BnMarketServer::new();
    let trade_server = BnTradeServer::new(ret.unwrap());
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

#[no_mangle]
pub extern "C" fn new_order(order_request: *const c_char) {
    let order_request_rust = c_char_to_string(order_request);
    let gateway_ref = context::get_trade_gateway();
    let mut gateway = gateway_ref.lock().unwrap();
    let ret = serde_json::from_str::<NewOrderRequest>(&order_request_rust);
    match ret {
        Ok(order) => {
            let ret = gateway.new_order(order);
            if ret.is_err() {
                panic!("{:?}", ret.unwrap_err());
            }
        },
        Err(e) => {
            panic!("{:?}", e);
        },
    }
}

#[no_mangle]
pub extern "C" fn cancel_order(symbol : *const c_char, order_id : *const c_char) {
    let symbol_rust = c_char_to_string(symbol);
    let order_id_rust = c_char_to_string(order_id);

    let gateway_ref = context::get_trade_gateway();
    let mut gateway = gateway_ref.lock().unwrap();

    let ret = gateway.cancel_order(&symbol_rust, &order_id_rust);
    if ret.is_err() {
        panic!("{}:{}:{:?}", symbol_rust, order_id_rust, ret.unwrap_err());
    }
}

#[no_mangle]
pub extern "C" fn cancel_orders(symbol : *const c_char) {
    let symbol_rust = c_char_to_string(symbol);

    let gateway_ref = context::get_trade_gateway();
    let mut gateway = gateway_ref.lock().unwrap();

    let ret = gateway.cancel_orders(&symbol_rust);
    if ret.is_err() {
        panic!("{}: {:?}", symbol_rust, ret.unwrap_err());
    }
}

#[no_mangle]
pub extern "C" fn get_positions(symbol : *const c_char)  -> Box<CString> {
    let symbol_rust = c_char_to_string(symbol);
    let gateway_ref = context::get_trade_gateway();
    let mut gateway = gateway_ref.lock().unwrap();
    let positions = gateway.get_positions(&symbol_rust);
    let json = serde_json::to_string(&positions).unwrap();
    Box::new(CString::new(json).unwrap())
}

#[no_mangle]
pub extern "C" fn get_account(asset : *const c_char) -> Box<CString> {
    let asset_rust = c_char_to_string(asset);
    let gateway_ref = context::get_trade_gateway();
    let mut gateway = gateway_ref.lock().unwrap();
    let account = gateway.get_account(&asset_rust);
    let json = serde_json::to_string(&account).unwrap();
    Box::new(CString::new(json).unwrap()) 
}

#[no_mangle]
pub extern "C" fn init_symbol_trade(sub_id: *const c_char, symbol: *const c_char, config: *const c_char, callback: extern "C" fn(*const c_char, *const c_char)) -> Box<CString> {
    let symbol_rust = c_char_to_string(symbol);
    let config_rust = c_char_to_string(config);
    let ret = serde_json::from_str::<SymbolConfig>(&config_rust);
    if ret.is_err() {
        panic!("{:?}", ret.unwrap_err());
    }

    let gateway_ref = context::get_trade_gateway();
    let mut gateway = gateway_ref.lock().unwrap();

    let ret = gateway.init_symbol(&symbol_rust, ret.unwrap());
    if ret.is_err() {
        panic!("{:?}", ret.unwrap_err());
    }
    let symbol_info = ret.unwrap();
    let rx = gateway.register_symbol(&symbol_rust);

    let sub_id_rust = CString::new(c_char_to_string(sub_id)).expect("CString failed");
    thread::spawn(move || {
        loop {
            if let Ok(data) = rx.recv() {
                match data {
                    AccountEvent::OrderTradeUpdate(order) => {
                        let datetime = DateTime::from_timestamp((order.order_trade_time/1000) as i64, 0).unwrap();
                        let order_event = OrderEvent {
                            symbol: order.symbol.clone(),
                            client_order_id: order.client_order_id.clone(),
                            side: order.side.clone(),
                            order_type: order.order_type.clone(),
                            original_quantity: order.original_quantity,
                            original_price: order.original_price,
                            average_price: order.average_price,
                            stop_price: order.stop_price,
                            execution_type: order.execution_type.clone(),
                            order_status: order.order_status.clone(),
                            order_last_filled_quantity: order.order_last_filled_quantity,
                            order_filled_accumulated_quantity: order.order_filled_accumulated_quantity,
                            last_filled_price: order.last_filled_price,
                            order_trade_time: datetime.format("%Y-%m-%d %H:%M:%S").to_string(),
                            trade_id: order.trade_id,
                        };

                        let json = serde_json::to_string(&order_event).unwrap();
                        let json_rust = CString::new(json).expect("CString failed");
                        callback(sub_id_rust.as_ptr(), json_rust.as_ptr());
                    },
                    _ => {},
                }
            }
        }
    });
    let json = serde_json::to_string(&symbol_info).unwrap();
    Box::new(CString::new(json).unwrap()) 
}
