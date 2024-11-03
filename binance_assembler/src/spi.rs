


use std::error::Error;
use std::os::raw::*;
use std::ffi::CString;
use std::str::FromStr;
use std::thread;
use binance::bn_market_server::BnMarketServer;
use binance::bn_trade_server::{BnTradeServer, Config};
use binance_future_connector::trade::enums::*;
use binance_future_connector::trade::new_order::NewOrderRequest;
use common::c::*;
use market::market_server::{KLine, MarketData, Tick};
use crate::context;
use crate::c_model::*;
use rust_decimal::Decimal;

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
    println!("start to initialize binance...{:?}", ret.as_ref().unwrap());
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

    println!("{}", order_request_rust);
    let ret = serde_json::from_str::<NewOrderRequest>(&order_request_rust);
    println!("{:?}", ret);
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
        panic!("{:?}", ret.unwrap_err());
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

/*
fn convert_c_neworder(order_request: *const CNewOrderRequest) -> Result<NewOrderRequest, Box<dyn Error>> {
    let order_request_ref = unsafe { &*order_request };

    let request = NewOrderRequest {
        symbol: c_char_to_string(order_request_ref.symbol),
        side: Side::from_str(&c_char_to_string(order_request_ref.side))?,
        position_side: optional::<PositionSide>(&c_char_to_string(order_request_ref.position_side))?,
        type_: OrderType::from_str(&c_char_to_string(order_request_ref.type_))?,
        reduce_only: optional::<String>(&c_char_to_string(order_request_ref.reduce_only))?,
        quantity: Decimal::from_f64_retain(order_request_ref.quantity),
        price: Decimal::from_f64_retain(order_request_ref.price),
        new_client_order_id: optional::<String>(&c_char_to_string(order_request_ref.new_client_order_id))?,
        stop_price: Decimal::from_f64_retain(order_request_ref.stop_price),
        close_position: optional::<String>(&c_char_to_string(order_request_ref.close_position))?,
        activation_price: Decimal::from_f64_retain(order_request_ref.activation_price),
        callback_rate: Decimal::from_f64_retain(order_request_ref.callback_rate),
        time_in_force: optional::<TimeInForceType>(&c_char_to_string(order_request_ref.time_in_force))?,
        working_type: optional::<String>(&c_char_to_string(order_request_ref.working_type))?,
        price_protect: optional::<String>(&c_char_to_string(order_request_ref.price_protect))?,
        new_order_resp_type: optional::<NewOrderResponseType>(&c_char_to_string(order_request_ref.new_order_resp_type))?,
        price_match: optional::<PriceMatchType>(&c_char_to_string(order_request_ref.price_match))?,
        self_trade_prevention_mode: optional::<String>(&c_char_to_string(order_request_ref.self_trade_prevention_mode))?,
        good_till_date: None,
        recv_window: None,
    };
    Ok(request)
} */

