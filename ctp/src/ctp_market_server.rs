#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use common::error::AppError;
use common::thread::{Handler, InteractiveThread};
use market::kline::KLineCombiner;
use super::ctp_market_spi::Spi;
use market::market_server::{KLine, MarketData, MarketServer};
use libctp_sys::*;
use std::ffi::{CStr, CString};
use std::os::raw::*;
use common::msmc::*;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use common::thread::Rx;


struct SafePointer<T>(*mut T);

unsafe impl<T> Send for SafePointer<T> {}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    flow_path: String,
    is_udp: bool,
    is_multicast: bool,
    front_addr: Vec<String>,
}

pub struct MDApi {
    api: Rust_CThostFtdcMdApi,
    spi: Option<SafePointer<Rust_CThostFtdcMdSpi>>,
    config: Config,
}


impl MDApi {
    pub fn get_version() -> String {
        let cs = unsafe { CStr::from_ptr(CThostFtdcMdApi::GetApiVersion()) };
        cs.to_string_lossy().into()
    }

    pub fn new(config: &Config) -> Self {
        let cs = std::ffi::CString::new(config.flow_path.as_bytes()).unwrap();
        let api = unsafe {
            Rust_CThostFtdcMdApi::new(CThostFtdcMdApi::CreateFtdcMdApi(
                cs.as_ptr(),
                config.is_udp,
                config.is_multicast,
            ))
        };
        Self {
            api,
            spi: None,
            config: config.clone(),
        }
    }

    fn req_init(&mut self) -> Subscription<MarketData> {
        let mut top = Subscription::top();
        let outer_subscription = top.subscribe();

        self.register(Spi::new(top));

        for addr in &self.config.front_addr {
            let cs = CString::new(addr.as_bytes()).unwrap();
            unsafe {
                self.api.RegisterFront(cs.as_ptr() as *mut _);
            }
        }
        unsafe {
            self.api.Init();
        }
        outer_subscription
    }

    fn req_user_login(&mut self) -> Result<(), String> {
        let mut loginfield = CThostFtdcReqUserLoginField {
            TradingDay: Default::default(),
            BrokerID: Default::default(),
            UserID: Default::default(),
            Password: [0i8; 41],
            UserProductInfo: Default::default(),
            InterfaceProductInfo: Default::default(),
            ProtocolInfo: Default::default(),
            MacAddress: Default::default(),
            OneTimePassword: [0i8; 41],
            ClientIPAddress: [0; 33],
            LoginRemark: [0i8; 36],
            ClientIPPort: Default::default(),
            reserve1: [0; 16],
        };

        unsafe {
            self.api.ReqUserLogin(&mut loginfield, 1);
        }
        Ok(())
    }

    fn check_connected(&mut self, subscription: &Subscription<MarketData>) -> Result<(), String> {
        let mut should_break = false;
        loop {
            let ret = subscription.recv_timeout(5,  &mut |event| {
                match event {
                    Some(data) => {
                        match data {
                            MarketData::Connected => {
                                should_break = true;
                            },
                            _ => {}
                        }
                    },
                    None => {
                    },
                }
            });
            if ret.is_err() {
                return Err("Error happened when connecting to market server".to_string());
            } else {
                if ret.unwrap().is_none() {
                    return Err("Closed connection of market server".to_string());
                }
            }
            if should_break {
                break;
            }
        }
        Ok(())
    }

    fn check_logined(&mut self, subscription: &Subscription<MarketData>) -> Result<(), String> {
        let mut should_break = false;
        loop {
            let ret = subscription.recv_timeout(5,  &mut |event| {
                match event {
                    Some(data) => {
                        match data {
                            MarketData::UserLogin => {
                                should_break = true;
                            },
                            _ => {}
                        }
                    },
                    None => {
                    },
                }
            });
            if ret.is_err() {
                return Err("Error happened when logining to market server".to_string());
            } else {
                if ret.unwrap().is_none() {
                    return Err("Closed connection of market server".to_string());
                }
            }
            if should_break {
                break;
            }
        }
        Ok(())
    }

    pub fn start(&mut self) -> Result<Subscription<MarketData>, String> {
        let subscription = self.req_init();
        let ret = self.check_connected(&subscription);
        if ret.is_err() {
            return Err(ret.unwrap_err());
        }

        self.req_user_login()?;
        let ret = self.check_logined(&subscription);
        if ret.is_err() {
            return Err(ret.unwrap_err());
        }
        Ok(subscription)
    }

    pub fn subscribe_market_data(&mut self, codes: &[&str], is_unsub: bool) -> Result<(), String> {
        let len = codes.len() as c_int;
        let arr_cstring: Vec<CString> = codes
            .iter()
            .map(|s| CString::new(s.as_bytes()).unwrap())
            .collect();
        let arr_cstr: Vec<*mut c_char> = arr_cstring
            .iter()
            .map(|s| s.as_ptr() as *mut c_char)
            .collect();
        let ptr = arr_cstr.as_ptr() as *mut *mut c_char;
        let rtn = if is_unsub {
            unsafe { self.api.UnSubscribeMarketData(ptr, len) }
        } else {
            unsafe { self.api.SubscribeMarketData(ptr, len) }
        };
        if rtn != 0 {
            return Err(format!(
                "Fail to req `md_api_subscribe_market_data`: {}",
                rtn
            ));
        }
        Ok(())
    }

    fn register<S: Rust_CThostFtdcMdSpi_Trait>(&mut self, spi: S) {
        if let Some(spi) = self.spi.take() {
            Self::drop_spi(spi);
        }

        let spi: Box<Box<dyn Rust_CThostFtdcMdSpi_Trait>> = Box::new(Box::new(spi));
        let ptr = Box::into_raw(spi) as *mut _ as *mut c_void;

        let spi_stub = unsafe { Rust_CThostFtdcMdSpi::new(ptr) };
        let spi: *mut Rust_CThostFtdcMdSpi = Box::into_raw(Box::new(spi_stub));
        unsafe {
            self.api.RegisterSpi(spi as _);
        }

        self.spi = Some(SafePointer(spi));
    }

    fn drop_spi(spi: SafePointer<Rust_CThostFtdcMdSpi>) {
        let mut spi = unsafe { Box::from_raw(spi.0) };
        unsafe {
            spi.destruct();
        }
    }
}

impl Drop for MDApi {
    fn drop(&mut self) {
        unsafe {
            self.api.destruct();
        }
        if let Some(spi) = self.spi.take() {
            Self::drop_spi(spi);
        }
    }
}

#[derive(Clone)]
pub struct MarketTopic {
    pub symbol: String,
    pub interval: String,
}
pub struct CtpMarketServer {
   topics: Vec<MarketTopic>,
   subscription: Subscription<MarketData>,
}

impl CtpMarketServer {
    pub fn new() -> Self {
        CtpMarketServer {
            topics: Vec::new(),
            subscription: Subscription::top(),
        }
    }
}

static mut MDAPI: Option<MDApi> = None;

impl MarketServer for CtpMarketServer {
    fn connect(&mut self, _prop : &HashMap<String, String>) -> Result<Subscription<MarketData>, AppError> {
        let mut mdapi = MDApi::new(&Config {
            flow_path: "".into(),
            front_addr: vec![_prop.get("front_addr").unwrap_or(&"".to_string()).clone()],
            ..Default::default()
        });
        let mut subscription = mdapi.start().unwrap();
        let outer_subscription = subscription.subscribe();
        self.subscription = subscription;

        unsafe {
            MDAPI = Some(mdapi);
        }
        Ok(outer_subscription)
    }

    fn subscribe_tick(&mut self, symbol: &str) {
        let mut found = false;
        for topic in self.topics.iter() {
            if topic.symbol == symbol {
                found = true;
                break;
            }
        }
        if !found {
            let topic = MarketTopic {
                symbol: symbol.to_string(),
                interval: "".to_string(),
            };
            self.topics.push(topic);
        }
    }

    fn subscribe_kline(&mut self, symbol: &str, interval: &str) {
        let mut found = false;
        for topic in self.topics.iter() {
            if topic.symbol == symbol && topic.interval == interval {
                
                found = true;
                break;
            }
        }
        if !found {
            let topic = MarketTopic {
                symbol: symbol.to_string(),
                interval: interval.to_string(),
            };
            self.topics.push(topic);
        }
    }

    fn start(self) -> Handler<()> {
        let closure = move |rx: Rx<String>| {
            let mut tick_set = HashSet::new();
            for topic in self.topics.iter() {
                if topic.interval == "" {
                    if !tick_set.contains(topic.symbol.as_str()) {
                        unsafe {
                            MDAPI.as_mut().unwrap().subscribe_market_data(&[topic.symbol.as_str()], false).unwrap();
                        }
                        tick_set.insert(topic.symbol.to_string());
                    }
                } 
            }

            let mut combiner_map:HashMap<String, KLineCombiner> = HashMap::new();
            self.subscription.stream(&mut |event| {
                let command = rx.try_recv();
                if let Ok(cmd) = command {
                    if cmd == "QUIT" {
                        return false;
                    }
                }

                if let Some(data) = event {
                    match data {
                        MarketData::Tick(t) => {
                            self.subscription.send(&Some(MarketData::Tick(t.clone())));
                            for topic in self.topics.iter() {
                                if topic.symbol == t.symbol {
                                    let combiner = combiner_map.entry(format!("{}_{}", topic.symbol, topic.interval)).or_insert(KLineCombiner::new(topic.interval.as_str(), 100, Some(21)));
                                    let kline = KLine {
                                        symbol: t.symbol.clone(),
                                        datetime: t.datetime.clone(),
                                        interval: topic.interval.clone(),
                                        open: t.open,
                                        high: t.high,
                                        low: t.low,
                                        close: t.close,
                                        volume: t.volume as i32,
                                        turnover: t.turnover,
                                    };
                                    let mut new_kline = combiner.combine_tick(&kline, true);
                                    if let Some(kline) = new_kline.take() {
                                        let _ = self.subscription.send(&Some(MarketData::Kline(kline)));
                                    }
                                }
                            }
                        },
                        _ => {
                        },
                    }
                    true
                } else {
                    false
                }
            });
        };
        let handler = InteractiveThread::spawn(closure);
        handler
    }

    fn close(&mut self) {}
}