#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use common::error::AppError;
use super::ctp_market_spi::Spi;
use market::market_server::{MarketServer, MarketData};
use libctp_sys::*;
use std::sync::{Mutex, Arc, RwLock};
use std::ffi::{CStr, CString};
use std::os::raw::*;
use common::msmc::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;


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
    subscription: Arc<RwLock<Subscription<MarketData>>>,
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
            subscription : Arc::new(RwLock::new(Subscription::top())),
        }
    }

    fn req_init(&mut self) -> Result<Subscription<MarketData>, String> {
        let mut top = Subscription::top();
        top.publish_to_under(self.subscription.write().as_mut().unwrap());

        let outer_subscription = top.subscribe_with_filter(Box::new(|data| {
            match data {
                MarketData::Tick(_) => {
                    true
                },
                _ => {
                    false
                },
            }  
        }));
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

        Ok(outer_subscription)
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

    fn check_connected(&mut self) -> Result<(), String> {
        let mut should_break = false;
        let subscription = self.subscription.write().unwrap();
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

    fn check_logined(&mut self) -> Result<(), String> {
        let mut should_break = false;
        let subscription = self.subscription.write().unwrap();
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
        let outter_subscription = self.req_init()?;
        let ret = self.check_connected();
        if ret.is_err() {
            return Err(ret.unwrap_err());
        }

        self.req_user_login()?;
        let ret = self.check_logined();
        if ret.is_err() {
            return Err(ret.unwrap_err());
        }
        Ok(outter_subscription)
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

pub struct CtpMarketServer {
}

impl CtpMarketServer {
    pub fn new() -> Self {
        CtpMarketServer {
        }
    }
}

static mut MDAPI: Option<Arc<Mutex<MDApi>>> = None;

impl MarketServer for CtpMarketServer {
    fn connect(&mut self, _prop : &HashMap<String, String>) -> Result<Subscription<MarketData>, AppError> {
        env_logger::init();
        let mut mdapi = MDApi::new(&Config {
            flow_path: "".into(),
            front_addr: vec![_prop.get("front_addr").unwrap_or(&"".to_string()).clone()],
            ..Default::default()
        });
        let subscription = mdapi.start().unwrap();
        unsafe {
            MDAPI = Some(Arc::new(Mutex::new(mdapi)));
        }
        Ok(subscription)
    }

    fn subscribe(&mut self, symbol: &str) -> Result<(), AppError> {
        unsafe {
            let mdapi = MDAPI.as_ref().unwrap().clone();
            mdapi.lock().unwrap().subscribe_market_data(&[symbol], false).unwrap();
        }
        Ok(())
    }
}