#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use common::thread::{Handler, InteractiveThread, Rx};
use libctp_sys::*;

use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::os::raw::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread::sleep;
use std::time::Duration;
use std::vec;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex, RwLock};
use trade::trade_server::*;
use common::{c::*, msmc::Subscription, error::AppError};
use crate::model::{Account, CancelOrderRequest, Config, NewOrderRequest, Position, TradeEvent};

use super::ctp_code::*;
use super::ctp_trade_cpi::Spi;
use std::cmp::min;
use log::*;

struct SafePointer<T>(*mut T);

unsafe impl<T> Send for SafePointer<T> {}

#[derive(Debug, Serialize, Deserialize, Copy, Clone, Eq, PartialEq)]
pub enum Resume {
    Restart = THOST_TE_RESUME_TYPE_THOST_TERT_RESTART as _,
    Resume = THOST_TE_RESUME_TYPE_THOST_TERT_RESUME as _,
    Quick = THOST_TE_RESUME_TYPE_THOST_TERT_QUICK as _,
}

pub struct TDApi {
    api: Rust_CThostFtdcTraderApi,
    spi: Option<SafePointer<Rust_CThostFtdcTraderSpi>>,
    pub config: Config,
    pub subscription: Subscription<TradeEvent>,
}
impl TDApi {
    fn send_request<F>(f: &mut F) -> Result<(), String> 
        where F: FnMut() -> i32 {
        let ret = f();
        match ret {
            0 => Ok(()),
            -1 => Err(format!(
                "Fail to connect network"
            )),
            -2 => Err(format!(
                "Pending request exceeds"
            )),
            -3 => Err(format!(
                "Quota per second exceeds"
            )),
            _ => Err(format!(
                "Unknown result from trade api {}", ret
            )),
        }
    }

    pub fn get_version() -> String {
        let cs = unsafe { CStr::from_ptr(CThostFtdcTraderApi::GetApiVersion()) };
        cs.to_string_lossy().into()
    }

    pub fn new(config: Config) -> Self {
        let cs = std::ffi::CString::new(config.flow_path.as_bytes()).unwrap();
        let api = unsafe {
            Rust_CThostFtdcTraderApi::new(CThostFtdcTraderApi_CreateFtdcTraderApi(cs.as_ptr()))
        };
        Self {
            api,
            spi: None,
            config,
            subscription: Subscription::top(),
        }
    }

    fn req_user_login(&mut self, request_id: i32) -> Result<(), String> {
        let mut request = CThostFtdcReqUserLoginField {
            BrokerID: string_to_c_char::<11>(self.config.broker_id.clone()),
            UserID: string_to_c_char::<16>(self.config.user_id.clone()),
            Password: string_to_c_char::<41>(self.config.password.clone()),

            TradingDay: string_to_c_char::<9>("".to_string()),
            UserProductInfo: string_to_c_char::<11>("".to_string()),
            InterfaceProductInfo: string_to_c_char::<11>("".to_string()),
            ProtocolInfo: string_to_c_char::<11>("".to_string()),
            MacAddress: string_to_c_char::<21>("".to_string()),
            OneTimePassword: string_to_c_char::<41>("".to_string()),
            reserve1: string_to_c_char::<16>("".to_string()),
            LoginRemark: string_to_c_char::<36>("".to_string()),
            ClientIPPort: 0 as c_int,
            ClientIPAddress: string_to_c_char::<33>("".to_string()),
        };
        
        Self::send_request(&mut move || unsafe {
            self.api.ReqUserLogin(&mut request, request_id)
        })
    }

    fn req_settlement_info_confirm(&mut self, settlement_id : i32, request_id: i32) -> Result<(), String>{
        let mut request = CThostFtdcSettlementInfoConfirmField {
            BrokerID: string_to_c_char::<11>(self.config.broker_id.clone()),
            InvestorID: string_to_c_char::<13>(self.config.user_id.clone()),
            ConfirmDate: string_to_c_char::<9>("".to_string()),
            ConfirmTime: string_to_c_char::<9>("".to_string()),
            SettlementID: settlement_id as c_int,
            AccountID: string_to_c_char::<13>("".to_string()),
            CurrencyID: string_to_c_char::<4>("".to_string()),
        };

        Self::send_request(&mut move || unsafe {
            self.api.ReqSettlementInfoConfirm(&mut request, 
                request_id)
        })
    }

    fn req_order_insert(&mut self, order: NewOrderRequest, unit_id: &str, request_id: i32) -> Result<(), String> {
        let order_type = OrderType::from_string(ORDER_TYPE.as_ref().get(&order.order_type).unwrap());

        let mut request = CThostFtdcInputOrderField {
            BrokerID: string_to_c_char::<11>(self.config.broker_id.clone()),
            InvestorID: string_to_c_char::<13>(self.config.user_id.clone()),
            InstrumentID: string_to_c_char::<81>(order.symbol.to_string()),
            OrderRef: string_to_c_char::<13>(order.order_ref.to_string()),

            CombOffsetFlag: string_to_c_char::<5>(OFFSET.as_ref().get(&order.offset).unwrap().to_string()),
            CombHedgeFlag: string_to_c_char::<5>("1".to_string()),
            ExchangeID: string_to_c_char::<9>(order.exchange_id.to_string()),
            RequestID: request_id as c_int,
            VolumeTotalOriginal: order.volume_total as c_int,
            IsAutoSuspend: 0 as c_int,
            IsSwapOrder: 0 as c_int,
            OrderPriceType: order_type.price_type as c_char,

            Direction: DIRECTION.as_ref().get(&order.direction).unwrap().chars().next().unwrap() as c_char,
            TimeCondition: order_type.time_condition as c_char,
            VolumeCondition: order_type.volume_condition as c_char,
            InvestUnitID: string_to_c_char::<17>(unit_id.to_string()),
            UserID: string_to_c_char::<16>(unit_id.to_string().to_string()),

            ContingentCondition: '1' as c_char,
            ForceCloseReason: '0' as c_char,
            LimitPrice: order.limit_price,
            StopPrice: 0.0,
            MinVolume: 1 as c_int,

            reserve2: string_to_c_char::<16>("".to_string()),
            reserve1: string_to_c_char::<31>("".to_string()),
            GTDDate: string_to_c_char::<9>("".to_string()),
            BusinessUnit: string_to_c_char::<21>("".to_string()),
            UserForceClose: 0 as c_int,
            AccountID: string_to_c_char::<13>("".to_string()),
            CurrencyID: string_to_c_char::<4>("".to_string()),
            ClientID: string_to_c_char::<11>("".to_string()),
            MacAddress: string_to_c_char::<21>("".to_string()),
            IPAddress: string_to_c_char::<33>("".to_string()),
        };
        Self::send_request(&mut move || unsafe {
            self.api.ReqOrderInsert(&mut request, request_id)
        })
    }
    
    fn req_order_action(&mut self, request: CancelOrderRequest, request_id: i32) -> Result<(), String> {
        let mut request = CThostFtdcInputOrderActionField {
            BrokerID: string_to_c_char::<11>(self.config.broker_id.clone()),
            InvestorID: string_to_c_char::<13>(self.config.user_id.clone()),

            UserID: string_to_c_char::<16>(request_id.to_string()),
            InstrumentID: string_to_c_char::<81>(request.symbol.clone()),
            ExchangeID: string_to_c_char::<9>(request.exchange.clone()),
            OrderSysID: string_to_c_char::<21>(request.order_id.to_string()),
            OrderRef: string_to_c_char::<13>("".to_string()),
            OrderActionRef: 0 as c_int,
            RequestID: request_id as c_int,

            ActionFlag: THOST_FTDC_AF_Delete as i8,
            FrontID: 0 as c_int,
            SessionID: 0 as c_int,
            InvestUnitID: string_to_c_char::<17>("".to_string()),
            LimitPrice: 0.0 as f64,
            VolumeChange: 0 as c_int,
            reserve1: string_to_c_char::<31>("".to_string()),
            reserve2: string_to_c_char::<16>("".to_string()),
            MacAddress: string_to_c_char::<21>("".to_string()),
            IPAddress: string_to_c_char::<33>("".to_string()),
        };
        Self::send_request(&mut move || unsafe {
            self.api.ReqOrderAction(&mut request, request_id)
        })
    }

    fn req_qry_investor_position(&mut self, request_id: i32) -> Result<(), String> {
        let mut request = CThostFtdcQryInvestorPositionField {
            BrokerID: string_to_c_char::<11>(self.config.broker_id.clone()),
            InvestorID: string_to_c_char::<13>(self.config.user_id.clone()),
            reserve1: string_to_c_char::<31>("".to_string()),
            ExchangeID: string_to_c_char::<9>("".to_string()),
            InvestUnitID: string_to_c_char::<17>("".to_string()),
            InstrumentID: string_to_c_char::<81>("".to_string()),
        };
        
        Self::send_request(&mut move || unsafe {
            self.api.ReqQryInvestorPosition(&mut request, request_id)
        })
    }

    fn req_qry_trading_account(&mut self, request_id: i32) -> Result<(), String> {
        let mut request = CThostFtdcQryTradingAccountField {
            BrokerID: string_to_c_char::<11>(self.config.broker_id.clone()),
            InvestorID: string_to_c_char::<13>(self.config.user_id.clone()),
            CurrencyID: string_to_c_char::<4>("".to_string()),
            BizType: '0' as c_char,
            AccountID: string_to_c_char::<13>("".to_string()),
        };
        
        Self::send_request(&mut move || unsafe {
            self.api.ReqQryTradingAccount(&mut request, request_id)
        })
    }
    /*
    fn req_qry_order(&mut self, request_id: i32) -> Result<(), String> {
        let mut request = CThostFtdcQryOrderField {
            BrokerID: string_to_c_char::<11>(self.config.broker_id.clone()),
            InvestorID: string_to_c_char::<13>(self.config.user_id.clone()),
            reserve1: string_to_c_char::<31>("".to_string()),
            ExchangeID: string_to_c_char::<9>("".to_string()),
            OrderSysID: string_to_c_char::<21>("".to_string()),
            InsertTimeStart: string_to_c_char::<9>("".to_string()),
            InsertTimeEnd: string_to_c_char::<9>("".to_string()),
            InvestUnitID: string_to_c_char::<17>("".to_string()),
            InstrumentID: string_to_c_char::<81>("".to_string()),
        };
        
        Self::send_request(&mut move || unsafe {
            self.api.ReqQryOrder(&mut request, request_id)
        })
    }

    fn req_qry_trade(&mut self, request_id: i32) -> Result<(), String> {
        let mut request = CThostFtdcQryTradeField {
            BrokerID: string_to_c_char::<11>(self.config.broker_id.clone()),
            InvestorID: string_to_c_char::<13>(self.config.user_id.clone()),
            reserve1: string_to_c_char::<31>("".to_string()),
            ExchangeID: string_to_c_char::<9>("".to_string()),
            TradeID: string_to_c_char::<21>("".to_string()),
            TradeTimeStart: string_to_c_char::<9>("".to_string()),
            TradeTimeEnd: string_to_c_char::<9>("".to_string()),
            InvestUnitID: string_to_c_char::<17>("".to_string()),
            InstrumentID: string_to_c_char::<81>("".to_string()),
        };
        Self::send_request(&mut move || unsafe {
            self.api.ReqQryTrade(&mut request, request_id)
        })
    }
 */
    fn drop_spi(spi: SafePointer<Rust_CThostFtdcTraderSpi>) {
        let mut spi: Box<Rust_CThostFtdcTraderSpi> = unsafe { Box::from_raw(spi.0) };
        unsafe {
            spi.destruct();
        }
    }

    fn register<S: Rust_CThostFtdcTraderSpi_Trait>(&mut self, spi: S) {
        if let Some(spi) = self.spi.take() {
            Self::drop_spi(spi);
        }

        let spi: Box<Box<dyn Rust_CThostFtdcTraderSpi_Trait>> = Box::new(Box::new(spi));
        let ptr = Box::into_raw(spi) as *mut _ as *mut c_void;

        let spi_stub = unsafe { Rust_CThostFtdcTraderSpi::new(ptr) };
        let spi: *mut Rust_CThostFtdcTraderSpi = Box::into_raw(Box::new(spi_stub));
        unsafe {
            self.api.RegisterSpi(spi as _);
        }

        self.spi = Some(SafePointer(spi));
    }

    pub fn req_init(&mut self) -> Result<Subscription<TradeEvent>, String> {
        let mut spi = Spi::new();
        let subscription = spi.subscription.subscribe();
        self.register(spi);

        let cs = CString::new(self.config.front_addr.as_bytes()).unwrap();
        unsafe {
            self.api.RegisterFront(cs.as_ptr() as *mut _);
        }

        unsafe {
            self.api
                .SubscribePrivateTopic(Resume::Quick as _);
            self.api
                .SubscribePublicTopic(Resume::Quick as _);
        }

        unsafe {
            self.api.Init();
        }
        Ok(subscription)
    }
    
    fn check_connected(&mut self, subscription: &Subscription<TradeEvent>) -> Result<(), String> {
        let mut should_break = false;
        loop {
            let ret = subscription.recv_timeout(5,  &mut |event| {
                match event {
                    Some(data) => {
                        info!("Received from trade server {:?}", data);
                        match data {
                            TradeEvent::Connected => {
                                should_break = true;
                            },
                            _ => {},
                        }
                    },
                    None => { 
                        should_break = true;
                    }
                }
            });
            if ret.is_err() {
                return Err(format!("Error happened when connecting to trade server: {:?}", ret.unwrap_err()));
            } else {
                if ret.unwrap().is_none() {
                    return Err("Closed connection of trade server".to_string());
                }
            }
            if should_break {
                break;
            }
        }
        Ok(())
    }

    fn check_logined(&mut self, subscription: &Subscription<TradeEvent>) -> Result<(), String> {
        let mut should_break = false;
        loop {
            let ret = subscription.recv_timeout(5,  &mut |event| {
                match event {
                    Some(data) => {
                        match data {
                            TradeEvent::UserLogin() => {
                                should_break = true;
                            },
                            _ => {},
                        }
                    },
                    None => { 
                        should_break = true;
                    }
                }
            });
            if ret.is_err() {
                return Err("Error happened when logining to trade server".to_string());
            } else {
                if ret.unwrap().is_none() {
                    return Err("Closed connection of trade server".to_string());
                }
            }
            if should_break {
                break;
            }
        }
        Ok(())
    }

    pub fn start(&mut self) -> Result<Subscription<TradeEvent>, String> {
        let subscription = self.req_init()?;
        let ret = self.check_connected(&subscription);
        if ret.is_err() {
            return Err(ret.unwrap_err());
        }

        self.req_user_login(0)?;
        let ret = self.check_logined(&subscription);
        if ret.is_err() {
            return Err(ret.unwrap_err());
        }

        self.req_settlement_info_confirm(0, 0)?;

        Ok(subscription)
    }
}

impl Default for Resume {
    fn default() -> Self {
        Self::Quick
    }
}

impl Drop for TDApi {
    fn drop(&mut self) {
        unsafe {
            self.api.destruct();
        }
        if let Some(spi) = self.spi.take() {
            Self::drop_spi(spi);
        }
    }
}

pub struct CtpTradeServer {
    tapi: Option<Arc<Mutex<TDApi>>>,
    config: Config,
    handler: Option<Handler<()>>,
    pub positions: Arc<RwLock<Vec<Position>>>,
    pub account: Arc<RwLock<Account>>,
    start_ticket: Arc<AtomicUsize>,
}

impl CtpTradeServer {
    pub fn new(config: Config) -> Self {
        CtpTradeServer {
            tapi: None,
            config,
            handler: None,
            positions: Arc::new(RwLock::new(vec![])),
            account: Arc::new(RwLock::new(Account {..Default::default()})),
            start_ticket: Arc::new(AtomicUsize::new(0)),
        }
    }
}

impl TradeServer for CtpTradeServer {
    type Event = TradeEvent;
    type OrderRequest = NewOrderRequest;
    type CancelOrderRequest = CancelOrderRequest;
    type Position = Position;
    type Account = Account;
    type SymbolConfig = ();
    type SymbolInfo = ();
    
    fn init(&mut self) -> Result<(), AppError> {
        Ok(())
    }

    fn start(&mut self) -> Result<Subscription<Self::Event>, AppError> {
        let start_ticket = self.start_ticket.fetch_add(1, Ordering::SeqCst);
        let start_ticket_ref = self.start_ticket.clone();
        let mut tdapi = TDApi::new(Config {
            flow_path: "".into(),
            nm_addr: "".into(),
            user_info: "".into(),
            product_info: "".into(),
            front_addr: format!("tcp://{}", self.config.front_addr.clone()),
            broker_id: self.config.broker_id.clone(),
            auth_code: self.config.auth_code.clone(),
            app_id: self.config.app_id.clone(),
            user_id: self.config.user_id.clone(),
            password: self.config.password.clone(),
            ..Default::default()
        });
        let mut subscription = tdapi.start().map_err(|e| AppError::new(-200, &e))?;
        self.tapi = Some(Arc::new(Mutex::new(tdapi)));

        let tapi_ref = self.tapi.as_ref().unwrap().clone();
        let _ = InteractiveThread::spawn(move |_rx| {
            loop {
                let mut tapi = tapi_ref.lock().unwrap();
                let _ = tapi.req_qry_investor_position(0);
                let _ = tapi.req_qry_trading_account(0);
                sleep(Duration::from_secs(3));
            }
        });

        let positions_ref = self.positions.clone();
        let account_ref = self.account.clone();

        let outer_subscription = subscription.subscribe();
        let continue_flag = true;
        let closure = move |_rx: Rx<String>| {
            let _ = subscription.stream(&mut move |event| {
                if start_ticket != start_ticket_ref.load(Ordering::SeqCst) - 1 {
                    return Ok(false);
                }
                match event {
                    Some(TradeEvent::PositionQuery(v)) => {
                        *positions_ref.write().unwrap() = v.clone();
                    },
                    Some(TradeEvent::AccountQuery(v)) => {
                        *account_ref.write().unwrap() = v.clone();
                    },
                    _ => {},
                }
                Ok(continue_flag)
            }, true);
        };
        self.handler = Some(InteractiveThread::spawn(closure));
        Ok(outer_subscription)
    }

    fn new_order(&mut self, request: Self::OrderRequest) -> Result<(), AppError> {
        let mut tapi = self.tapi.as_ref().unwrap().lock().unwrap();
        if request.offset == OFFSET_CLOSE.code && request.exchange_id == "SHFE" {
            let v = self.get_positions(&request.symbol);
            let mut last_day = 0;

            for p in v.iter() {
                if p.direction != request.direction {
                    last_day += p.position - p.today_position;
                }
            }

            let mut remain= request.volume_total;
            if last_day > 0 {
                let mut last_day_order = request.clone();
                last_day_order.offset = OFFSET_CLOSEYESTERDAY.code.to_string();
                last_day_order.volume_total = min(last_day, request.volume_total);
                remain -= last_day_order.volume_total;
                let _ = tapi.req_order_insert(last_day_order, "", 0);
            }
            if remain > 0 {
                let mut today_day_order = request.clone();
                today_day_order.volume_total = remain;
                let _ = tapi.req_order_insert(today_day_order, "", 0);
            }
        } else {
            let _ = tapi.req_order_insert(request.clone(), "", 0);
        }

        let _ = tapi.req_order_insert(request.clone(), "", 0);
        Ok(())
    }

    fn cancel_order(&mut self, request: CancelOrderRequest) -> Result<(), AppError> {
        let mut tapi = self.tapi.as_ref().unwrap().lock().unwrap();
        let _ = tapi.req_order_action(request, 0);
        Ok(())
    }

    fn cancel_orders(&mut self, _symbol: &str) -> Result<(), AppError> {
        Ok(())
    }

    fn get_positions(&self, symbol: &str) -> Vec<Self::Position> {
        let positions = self.positions.as_ref().read().unwrap();
        let mut position_map = HashMap::<String, Position>::new();

        for p in positions.iter() {
            if p.symbol == symbol {
                let mut op = position_map.get_mut(&p.direction);
                if let Some(p1) = op.as_mut() {
                    p1.position = p1.position + p.position;
                } else {
                    position_map.insert(p.direction.clone(), p.clone());
                }
            }
        }
        position_map.values().cloned().collect()
    }

    fn get_account(&self, _account_id: &str) -> Option<Self::Account> {
        let account = self.account.as_ref().read().unwrap();
        Some(account.clone())
    }

    fn init_symbol(&self, _symbol: &str, _config: Self::SymbolConfig)-> Result<Self::SymbolInfo, AppError> {
        Ok(())
    }

    fn close(&self) {
        self.start_ticket.fetch_add(1, Ordering::SeqCst);
    }
}