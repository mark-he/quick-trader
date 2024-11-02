


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
pub extern "C" fn subscribe_kline(sub_id : *const c_char, symbol : *const c_char, interval : *const c_char, count: i32, callback: extern "C" fn(*const c_char, *const CKLine)) -> CKLines {
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
                        let kline = convert_c_kline(k);
                        callback(sub_id_rust.as_ptr(), &kline);
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
    let mut v = Vec::with_capacity(klines.len());
    for k in klines {
        let kline = convert_c_kline(k);
        v.push(kline);
    }
    let len = v.len();
    let boxed_data = Box::new(v);
    
    CKLines {
        length: len,
        ptr: Box::leak(boxed_data).as_ptr(),
    }
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
                        let market_data = convert_c_tick(tick);
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

#[no_mangle]
pub extern "C" fn new_order(order_request: *const CNewOrderRequest) {
    let gateway_ref = context::get_trade_gateway();
    let mut gateway = gateway_ref.lock().unwrap();
    let ret = convert_c_neworder(order_request);
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
pub extern "C" fn get_positions(symbol : *const c_char) {
    let symbol_rust = c_char_to_string(symbol);
    let gateway_ref = context::get_trade_gateway();
    let mut gateway = gateway_ref.lock().unwrap();
    let positions = gateway.get_positions(&symbol_rust);
}

#[no_mangle]
pub extern "C" fn get_account(asset : *const c_char) {
    let asset_rust = c_char_to_string(asset);
    let gateway_ref = context::get_trade_gateway();
    let mut gateway = gateway_ref.lock().unwrap();
    let account = gateway.get_account(&asset_rust);    
}

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
}

fn convert_c_kline(kline: KLine) -> CKLine {
    let market_data: CKLine = CKLine {
        symbol: string_to_c_char::<100>(kline.symbol.clone()),
        interval: string_to_c_char::<100>(kline.interval.clone()),
        datetime: string_to_c_char::<100>(kline.datetime.clone()),
        open: kline.open,
        high: kline.high,
        low: kline.low,
        close: kline.close,
        volume: kline.volume,
        turnover: kline.turnover,
    };
    market_data
}

fn convert_c_tick(tick: Tick) -> CTick {
    let market_data: CTick = CTick {
        symbol: string_to_c_char::<100>(tick.symbol.clone()),
        trading_day: string_to_c_char::<100>(tick.trading_day.clone()),
        datetime: string_to_c_char::<100>(tick.datetime.clone()),
        open: tick.open,
        high: tick.high,
        low: tick.low,
        close: tick.close,
        volume: tick.volume,
        turnover: tick.turnover,
        open_interest: tick.open_interest,
        last_price: tick.last_price,
        bid_price1: tick.bid_price1,
        bid_price2: tick.bid_price2,
        bid_price3: tick.bid_price3,
        bid_price4: tick.bid_price4,
        bid_price5: tick.bid_price5,
        bid_volume1: tick.bid_volume1,
        bid_volume2: tick.bid_volume2,
        bid_volume3: tick.bid_volume3,
        bid_volume4: tick.bid_volume4,
        bid_volume5: tick.bid_volume5,
        ask_price1: tick.ask_price1,
        ask_price2: tick.ask_price2,
        ask_price3: tick.ask_price3,
        ask_price4: tick.ask_price4,
        ask_price5: tick.ask_price5,
        ask_volume1: tick.ask_volume1,
        ask_volume2: tick.ask_volume2,
        ask_volume3: tick.ask_volume3,
        ask_volume4: tick.ask_volume4,
        ask_volume5: tick.ask_volume5,
    };
    market_data
}