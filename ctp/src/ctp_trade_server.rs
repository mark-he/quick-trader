#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use libctp_sys::*;

use std::ffi::{CStr, CString};
use std::os::raw::*;
use std::{thread, vec};
use serde::{Deserialize, Serialize};
use std::sync::{Mutex, RwLock, Arc};
use trade::trade_server::*;
use common::{c::*, msmc::Subscription, error::AppError};
use super::ctp_code::*;
use super::ctp_trade_spi::Spi;
use tokio::time::{interval, Duration as TokioDuration};
use tokio::runtime::Runtime;
use std::cmp::min;

struct SafePointer<T>(*mut T);

unsafe impl<T> Send for SafePointer<T> {}

#[derive(Debug, Serialize, Deserialize, Copy, Clone, Eq, PartialEq)]
pub enum Resume {
    Restart = THOST_TE_RESUME_TYPE_THOST_TERT_RESTART as _,
    Resume = THOST_TE_RESUME_TYPE_THOST_TERT_RESUME as _,
    Quick = THOST_TE_RESUME_TYPE_THOST_TERT_QUICK as _,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    flow_path: String,
    front_addr: String,
    nm_addr: String,
    user_info: String,
    product_info: String,
    auth_code: String,
    app_id: String,
    public_resume: Resume,
    private_resume: Resume,

    broker_id: String,
    user_id: String,
    password: String,
}

pub struct TDApi {
    api: Rust_CThostFtdcTraderApi,
    spi: Option<SafePointer<Rust_CThostFtdcTraderSpi>>,
    pub(crate) config: Config,
    pub subscription: Arc<RwLock<Subscription<(i32, TradeData)>>>,
    pub session: Option<TradeSession>,
    pub positions : Option<Vec<Position>>,
    pub account: Option<Account>,
}
#[allow(unused)]
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
                "Unknown result from trade api{}", ret
            )),
        }
    }

    pub fn get_version() -> String {
        let cs = unsafe { CStr::from_ptr(CThostFtdcTraderApi::GetApiVersion()) };
        cs.to_string_lossy().into()
    }

    pub fn new(config: &Config) -> Self {
        let cs = std::ffi::CString::new(config.flow_path.as_bytes()).unwrap();
        let api = unsafe {
            Rust_CThostFtdcTraderApi::new(CThostFtdcTraderApi_CreateFtdcTraderApi(cs.as_ptr()))
        };
        Self {
            api,
            spi: None,
            config: config.clone(),
            session: None,
            subscription: Arc::new(RwLock::new(Subscription::top())),
            positions: None,
            account: None,
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

    fn req_order_insert(&mut self, order: &OrderInsert, unit_id: &str, request_id: i32) -> Result<(), String> {
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
    
    fn req_order_action(&mut self, action: &OrderAction, request_id: i32) -> Result<(), String> {
        let mut request = CThostFtdcInputOrderActionField {
            BrokerID: string_to_c_char::<11>(self.config.broker_id.clone()),
            InvestorID: string_to_c_char::<13>(self.config.user_id.clone()),

            UserID: string_to_c_char::<16>(request_id.to_string()),
            InstrumentID: string_to_c_char::<81>(action.symbol.to_string()),
            ExchangeID: string_to_c_char::<9>(action.exchange_id.to_string()),
            OrderSysID: string_to_c_char::<21>(action.sys_id.to_string()),
            OrderActionRef: action.action_ref as c_int,
            RequestID: request_id as c_int,

            ActionFlag: THOST_FTDC_AF_Delete as i8,
            OrderRef: string_to_c_char::<13>("".to_string()),
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

    /// destory `self.spi`, which created by `TDApi`
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

    pub fn req_init(&mut self) -> Result<Subscription<(i32, TradeData)>, String> {
        let mut top = Subscription::top();
        let outter_subscription = top.subscribe_with_filter(Box::new(|event|{
            match event {
                (_, TradeData::OnOrder(_)) | (_, TradeData::OnTrade(_)) => {
                    true
                },
                _ => false
            }
        }));

        top.publish_to_under(self.subscription.write().as_mut().unwrap());
        self.register(Spi::new(top));

        let cs = CString::new(self.config.front_addr.as_bytes()).unwrap();
        unsafe {
            self.api.RegisterFront(cs.as_ptr() as *mut _);
        }

        unsafe {
            self.api
                .SubscribePrivateTopic(self.config.private_resume as _);
            self.api
                .SubscribePublicTopic(self.config.public_resume as _);
        }

        unsafe {
            self.api.Init();
        }
        Ok(outter_subscription)
    }
    
    
    fn check_connected(&mut self) -> Result<(), String> {
        let mut should_break = false;
        let subscription = self.subscription.write().unwrap();
        loop {
            let ret = subscription.recv_timeout(5,  &mut |event| {
                match event {
                    Some(data) => {
                        match &data.1 {
                            TradeData::Connected => {
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
                return Err("Error happened when connecting to trade server".to_string());
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

    fn check_logined(&mut self) -> Result<(), String> {
        let mut should_break = false;
        let subscription = self.subscription.write().unwrap();
        loop {
            let ret = subscription.recv_timeout(5,  &mut |event| {
                match event {
                    Some(data) => {
                        match &data.1 {
                            TradeData::UserLogin(s) => {
                                self.session = Some(s.clone());
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

    pub fn start(&mut self) -> Result<Subscription<(i32, TradeData)>, String> {
        let outter_subscription = self.req_init()?;
        let ret = self.check_connected();
        if ret.is_err() {
            return Err(ret.unwrap_err());
        }

        self.req_user_login(0)?;
        let ret = self.check_logined();
        if ret.is_err() {
            return Err(ret.unwrap_err());
        }

        self.req_settlement_info_confirm(0, 0)?;
        
        self.standby();
        Ok(outter_subscription)
    }

    fn standby(&self) {
        let mut runtime = Runtime::new().unwrap();
        runtime.spawn(async {
            let mut interval = interval(TokioDuration::from_secs(1));  // 每隔 1 秒
            loop {
                interval.tick().await;
                unsafe {
                    let tdapi = TDAPI.as_ref().unwrap().clone();
                    let _ = tdapi.lock().unwrap().req_qry_investor_position(0);
                    let _ = tdapi.lock().unwrap().req_qry_trading_account(0);
                }
            }
        });

        let subscription_ref = self.subscription.clone();
        //stand by
        thread::spawn(move|| {
            let mut should_break = false; 
            let subscription = subscription_ref.read().unwrap();
            loop {
                subscription.recv(&mut |event|{
                    match event {
                        Some((_, TradeData::PositionQuery(v))) => {
                            unsafe {
                                let tdapi = TDAPI.as_ref().unwrap().clone();
                                tdapi.lock().unwrap().positions = Some(v.clone());
                            }
                        },
                        Some((_, TradeData::AccountQuery(v))) => {
                            unsafe {
                                let tdapi = TDAPI.as_ref().unwrap().clone();
                                tdapi.lock().unwrap().account = Some(v.clone());
                            }
                        },
                        _ => {},
                        None => {
                            should_break = true
                        }
                    }
                });
                if should_break {
                    break;;
                }
            }
        });
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
}

#[allow(unused)]
impl CtpTradeServer {
    pub fn new() -> Self {
        CtpTradeServer {
        }
    }
    
}

static mut TDAPI: Option<Arc<Mutex<TDApi>>> = None;

impl TradeServer for CtpTradeServer {
    fn connect(&mut self, config: &TradeConfig) -> Result<Subscription<(i32, TradeData)>, AppError> {
        eprintln!("api version: {}", TDApi::get_version());
        let mut tdapi = TDApi::new(&Config {
            flow_path: "".into(),
            nm_addr: "".into(),
            user_info: "".into(),
            product_info: "".into(),
            public_resume: Resume::Quick,
            private_resume: Resume::Quick,
            
            front_addr: config.front_addr.clone(),
            broker_id: config.broker_id.clone(),
            auth_code: config.auth_code.clone(),
            app_id: config.app_id.clone(),
            user_id: config.user_id.clone(),
            password: config.password.clone(),
            ..Default::default()
        });
        let subscription = tdapi.start().unwrap();
        unsafe {
            TDAPI = Some(Arc::new(Mutex::new(tdapi)));
        }
        Ok(subscription)
    }

    fn send_order(&mut self, order : &OrderInsert, unit_id: &str, request_id : i32) {
        if order.offset == OFFSET_CLOSE.code && order.exchange_id == "SHFE" {
            let v = self.get_positions(unit_id, &order.symbol);
            let mut last_day = 0;

            for p in v.iter() {
                if p.direction != order.direction {
                    last_day += p.position - p.today_position;
                }
            }

            let mut remain= order.volume_total;
            if last_day > 0 {
                let mut last_day_order = order.clone();
                last_day_order.offset = OFFSET_CLOSEYESTERDAY.code.to_string();
                last_day_order.volume_total = min(last_day, order.volume_total);
                remain -= last_day_order.volume_total;

                unsafe {
                    let tdapi = TDAPI.as_ref().unwrap().clone();
                    let _ = tdapi.lock().unwrap().req_order_insert(&last_day_order, unit_id, request_id);
                }
            }
            if remain > 0 {
                let mut today_day_order = order.clone();
                today_day_order.volume_total = remain;
                unsafe {
                    let tdapi = TDAPI.as_ref().unwrap().clone();
                    let _ = tdapi.lock().unwrap().req_order_insert(&today_day_order, unit_id, request_id);
                }
            }
        } else {
            unsafe {
                let tdapi = TDAPI.as_ref().unwrap().clone();
                let _ = tdapi.lock().unwrap().req_order_insert(&order, unit_id, request_id);
            }
        }

        unsafe {
            let tdapi = TDAPI.as_ref().unwrap().clone();
            let _ = tdapi.lock().unwrap().req_order_insert(order, unit_id, request_id);
        }
    }
    
    fn cancel_order(&mut self, action: &OrderAction, request_id : i32) {
        unsafe {
            let tdapi = TDAPI.as_ref().unwrap().clone();
            let _ = tdapi.lock().unwrap().req_order_action(action, request_id);
        }
    }

    fn get_positions(&self, unit_id: &str, symbol: &str) -> Vec<Position> {
        unsafe {
            let tdapi = TDAPI.as_ref().unwrap().clone();
            let tdapi_instance = tdapi.lock().unwrap();
            let v = tdapi_instance.positions.as_ref().unwrap();

            let mut ret = vec![];
            for p in v.iter() {
                if p.invest_unit_id == unit_id && p.symbol == symbol {
                    ret.push(p.clone());
                }
            }
            ret
        }
    }

    fn get_account(&self, _unit_id: &str) -> Account {
        unsafe {
            let tdapi = TDAPI.as_ref().unwrap().clone();
            let account = tdapi.lock().unwrap().account.as_ref().unwrap().clone();
            account
        }
    }

    fn session(&self) -> Option<TradeSession> {
        unsafe {
            let tdapi = TDAPI.as_ref().unwrap().clone();
            let session = tdapi.lock().unwrap().session.clone();
            session
        }
    }
}