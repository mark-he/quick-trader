

use std::os::raw::*;
use std::ffi::CString;
use std::thread;
use common::c::*;


#[repr(C)]
#[derive(Debug, Clone)]
pub struct CKLine {
    pub datetime: *const c_char,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: i32,
    pub turnover: f64,
}


#[repr(C)]
#[derive(Debug, Clone)]
pub struct CTick {
    pub symbol: *const c_char,
    pub datetime: *const c_char,
    pub trading_day: *const c_char,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: i32,
    pub turnover: f64,
    pub open_interest: f64,
    pub last_price: f64,
    pub bid_price1: f64,
    pub bid_price2: f64,
    pub bid_price3: f64,
    pub bid_price4: f64,
    pub bid_price5: f64,
    pub bid_volume1: i32,
    pub bid_volume2: i32,
    pub bid_volume3: i32,
    pub bid_volume4: i32,
    pub bid_volume5: i32,
    pub ask_price1: f64,
    pub ask_price2: f64,
    pub ask_price3: f64,
    pub ask_price4: f64,
    pub ask_price5: f64,
    pub ask_volume1: i32,
    pub ask_volume2: i32,
    pub ask_volume3: i32,
    pub ask_volume4: i32,
    pub ask_volume5: i32,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct COrder {
    pub order_ref: *const c_char,
    pub direction: *const c_char,
    pub offset: *const c_char,
    pub price: f64,
    pub volume_total_original: i32,   
    pub submit_status: *const c_char,
    pub order_type: *const c_char,
    pub sys_id: *const c_char,
    pub status: *const c_char,
    pub volume_traded: i32,
    pub volume_total: i32,
    pub status_msg: *const c_char,
    pub symbol: *const c_char,
    pub request_id: i32,
    pub invest_unit_id : *const c_char,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct COrderInsert {
    pub symbol: *const c_char,
    pub order_ref: *const c_char,
    pub offset: *const c_char,
    pub order_type: *const c_char,
    pub exchange_id: *const c_char,
    pub volume_total: i32,
    pub direction: *const c_char,
    pub limit_price: f64,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct COrderAction {
    pub symbol: *const c_char,
    pub action_ref: i32,
    pub exchange_id: *const c_char,
    pub sys_id: *const c_char,
}
#[repr(C)]
#[derive(Debug, Clone)]
pub struct CTrade {
    pub order_ref: *const c_char,
    pub trade_id: *const c_char,
    pub sys_id: *const c_char,
    pub direction: *const c_char,
    pub offset: *const c_char,
    pub price: c_double,
    pub volume: c_int,
    pub datetime: *const c_char,
    pub symbol: *const c_char,
}


#[repr(C)]
#[derive(Debug, Clone)]
pub struct CStatus {
    pub code: c_int,
    pub message: *const c_char,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CTradeConfig {
    pub front_addr: *const c_char,
    pub broker_id: *const c_char,
    pub auth_code: *const c_char,
    pub app_id: *const c_char,
    pub user_id: *const c_char,
    pub password: *const c_char,
}

use ctp::ctp_market_server::CtpMarketServer;
use market::market_gateway::*;
use market::market_server::*;
use backtest::backtest_market_server::*;
use ctp::ctp_trade_server::CtpTradeServer;
use std::collections::HashMap;
use backtest::backtest_trade_server::BacktestTradeServer;
use trade::trade_gateway::TradeGatewayHolder;
use trade::trade_server::*;

#[no_mangle]
pub extern "C" fn start() {
    let gateway_ref = MarketGatewayHolder::get_gateway();
    let gateway = gateway_ref.lock().unwrap();
    let trade_gateway_ref = TradeGatewayHolder::get_gateway();
    let trade_gateway = trade_gateway_ref.lock().unwrap();
    let _ = gateway.start();
    let _ = trade_gateway.start();
}

#[no_mangle]
pub extern "C" fn init_backtest() {
    println!("starting to init backtest");
    let backtest_server = BacktestMarketServer::new();
    MarketGatewayHolder::init(Box::new(backtest_server));

    let gateway_ref = MarketGatewayHolder::get_gateway();
    let mut gateway = gateway_ref.lock().unwrap();
    let _ = gateway.connect(&HashMap::new());
    let market_sub = gateway.get_tick_sub();
    //let _ = gateway.subscribe("m2501");

    let mut backtest_server = BacktestTradeServer::new();
    backtest_server.set_market_sub(market_sub);
    TradeGatewayHolder::init(Box::new(backtest_server));

    let trade_gateway_ref = TradeGatewayHolder::get_gateway();
    let mut trade_gateway = trade_gateway_ref.lock().unwrap();
    let _ = trade_gateway.connect(&TradeConfig {..Default::default()});

    println!("starting to init backtest ended");
}

#[allow(unused_variables)]
#[no_mangle]
pub extern "C" fn subscribe_trade(unit_id : *const c_char, on_order: extern "C" fn(*const c_char, *const COrder), on_trade: extern "C" fn(*const c_char, *const CTrade), on_status: extern "C" fn(*const c_char, *const CStatus)) {
    let rust_unit_id = c_char_to_string(unit_id);

    let gateway_ref = TradeGatewayHolder::get_gateway();
    let mut gateway = gateway_ref.lock().unwrap();

    let ret = gateway.subscribe(&rust_unit_id);

    let unit_id_rust = CString::new(c_char_to_string(unit_id)).expect("CString failed");
    match ret {
        Ok(rx) => {
            thread::spawn(move || {
                loop {
                    if let Ok(data) = rx.recv() {
                        
                        match data {
                            TradeData::OnOrder(order) => {
                                let order_ref = CString::new(order.order_ref).expect("CString failed");
                                let direction = CString::new(order.direction).expect("CString failed");
                                let offset = CString::new(order.offset).expect("CString failed");
                                let price = order.price;
                                let volume_total_original = order.volume_total_original as i32;  
                                let submit_status = CString::new(order.submit_status).expect("CString failed");
                                let order_type = CString::new(order.order_type).expect("CString failed");
                                let sys_id = CString::new(order.sys_id).expect("CString failed");
                                let status = CString::new(order.status).expect("CString failed");
                                let volume_traded = order.volume_traded as i32;
                                let volume_total = order.volume_total as i32;
                                let status_msg = CString::new(order.status_msg).expect("CString failed");
                                let symbol = CString::new(order.symbol).expect("CString failed");
                                let request_id = order.request_id;
                                let invest_unit_id = CString::new(order.invest_unit_id).expect("CString failed");
    
                                let c_order = COrder {
                                    order_ref: order_ref.as_ptr(),
                                    direction: direction.as_ptr(),
                                    offset: offset.as_ptr(),
                                    price,
                                    volume_total_original,   
                                    submit_status: submit_status.as_ptr(),
                                    order_type: order_type.as_ptr(),
                                    sys_id: sys_id.as_ptr(),
                                    status: status.as_ptr(),
                                    volume_traded,
                                    volume_total,
                                    status_msg: status_msg.as_ptr(),
                                    symbol: symbol.as_ptr(),
                                    request_id,
                                    invest_unit_id : invest_unit_id.as_ptr(),
                                };
                                on_order(unit_id_rust.as_ptr(), &c_order);
                            },
                            TradeData::OnTrade(trade) => {
                                let order_ref = CString::new(trade.order_ref).expect("CString failed");
                                let trade_id = CString::new(trade.trade_id).expect("CString failed");
                                let sys_id = CString::new(trade.sys_id).expect("CString failed");
                                let direction = CString::new(trade.direction).expect("CString failed");
                                let offset = CString::new(trade.offset).expect("CString failed");
                                let price = trade.price;
                                let volume = trade.volume as i32;
                                let datetime = CString::new(trade.datetime).expect("CString failed");
                                let symbol = CString::new(trade.symbol).expect("CString failed");
                                let c_trade = CTrade {
                                    order_ref: order_ref.as_ptr(),
                                    trade_id: trade_id.as_ptr(),
                                    sys_id: sys_id.as_ptr(),
                                    direction: direction.as_ptr(),
                                    offset: offset.as_ptr(),
                                    price,
                                    volume,
                                    datetime: datetime.as_ptr(),
                                    symbol: symbol.as_ptr(),
                                };
                                on_trade(unit_id_rust.as_ptr(), &c_trade);
                            },
                            TradeData::Error(request_id, message) => {
                                println!("{} - {}", request_id, message);
                            },
                            _ => {

                            },
                        }
                    }
                }
            });
        },
        Err(e) => {
            panic!("Error happened when subscribing trade server: {}", e.message);
        },
    }
}

#[no_mangle]
pub extern "C" fn subscribe_kline(sub_id : *const c_char, symbol : *const c_char, interval : *const c_char, callback: extern "C" fn(*const c_char, *const CKLine)) {
    let symbol_rust = c_char_to_string(symbol);
    let interval_rust = c_char_to_string(interval);

    let gateway_ref = MarketGatewayHolder::get_gateway();
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
    let gateway_ref = MarketGatewayHolder::get_gateway();
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
                        callback(sub_id_rust.as_ptr(), &market_data);
                    },
                    _ => {},
                }
            }
        }
    });

}

#[no_mangle]
pub extern "C" fn place_order(unit_id : *const c_char, order_insert: *const COrderInsert) -> i32 {
    let unit_id_rust = c_char_to_string(unit_id);
    let order_insert_ref = unsafe { &*order_insert };
    let order = OrderInsert {
        symbol: c_char_to_string(order_insert_ref.symbol),
        order_ref: c_char_to_string(order_insert_ref.order_ref),
        offset: c_char_to_string(order_insert_ref.offset),
        order_type: c_char_to_string(order_insert_ref.order_type),
        exchange_id: c_char_to_string(order_insert_ref.exchange_id),
        volume_total: (order_insert_ref.volume_total as u32).clone(),
        direction: c_char_to_string(order_insert_ref.direction),
        limit_price: order_insert_ref.limit_price.clone(),
    };
    let gateway_ref = TradeGatewayHolder::get_gateway();
    let mut gateway = gateway_ref.lock().unwrap();
    let request_id = gateway.send_order(&unit_id_rust, &order);
    request_id
}

#[no_mangle]
pub extern "C" fn cancel_order(unit_id : *const c_char, order_action: *const COrderAction) -> i32 {
    let unit_id_rust = c_char_to_string(unit_id);
    let order_action_ref = unsafe { &*order_action };
    let order = OrderAction {
        symbol: c_char_to_string(order_action_ref.symbol),
        action_ref: order_action_ref.action_ref,
        exchange_id: c_char_to_string(order_action_ref.exchange_id),
        sys_id: c_char_to_string(order_action_ref.sys_id),
    };
    let gateway_ref = TradeGatewayHolder::get_gateway();
    let mut gateway = gateway_ref.lock().unwrap();
    let request_id = gateway.cancel_order(&unit_id_rust, &order);
    request_id
}

#[no_mangle]
pub extern "C" fn init_ctp(config: *const CTradeConfig) {
    let config_ref = unsafe { &*config };
    let c = TradeConfig {
        front_addr: c_char_to_string(config_ref.front_addr),
        broker_id: c_char_to_string(config_ref.broker_id),
        auth_code: c_char_to_string(config_ref.auth_code),
        app_id: c_char_to_string(config_ref.app_id),
        user_id: c_char_to_string(config_ref.user_id),
        password: c_char_to_string(config_ref.password),
    };

    println!("starting to init ctp");
    let ctp_server = CtpMarketServer::new();
    MarketGatewayHolder::init(Box::new(ctp_server));

    let gateway_ref = MarketGatewayHolder::get_gateway();
    let mut gateway = gateway_ref.lock().unwrap();
    let _ = gateway.connect(&HashMap::new());

    let ctp_server = CtpTradeServer::new();
    TradeGatewayHolder::init(Box::new(ctp_server));

    let trade_gateway_ref = TradeGatewayHolder::get_gateway();
    let mut trade_gateway = trade_gateway_ref.lock().unwrap();
    let _ = trade_gateway.connect(&c);

    let _ = gateway.start();
    let _ = trade_gateway.start();

    println!("starting to init ctp ended");
}
