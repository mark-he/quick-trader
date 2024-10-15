use super::market_server::{MarketData, MarketServer};
use std::{collections::HashMap, thread::{self, JoinHandle}};
use common::{error::AppError, msmc::Subscription};
use std::sync::{Arc, Mutex, RwLock};
use crossbeam::channel::{self, Receiver, Sender};

type SharedGw = Arc<Mutex<MarketGateway>>;

static mut INSTANCE: Option<SharedGw> = None;
pub struct MarketGatewayHolder {
}

impl MarketGatewayHolder {
    pub fn init(server: Box<dyn MarketServer>) {
        unsafe {
            INSTANCE = Some(Arc::new(Mutex::new(MarketGateway {
                server: server,
                connected: false,
                subscription: None,
                subscribers: Vec::new(),
            })));
        }
    }

    pub fn get_gateway() -> SharedGw {
        unsafe {
            INSTANCE.as_ref().unwrap().clone()
        }
    }
}

#[derive(Clone)]
struct Subscriber {
    symbol: String,
    interval: String,
    sender: Sender<MarketData>,
}

pub struct MarketGateway {
    server: Box<dyn MarketServer>,
    connected: bool,
    subscription: Option<Arc<RwLock<Subscription<MarketData>>>>,
    subscribers : Vec<Subscriber>,
}

impl MarketGateway {
    pub fn connect(&mut self, prop : &HashMap<String, String>)  -> Result<(), AppError> {
        if !self.connected {
            self.subscription = Some(Arc::new(RwLock::new(self.server.connect(prop)?)));
            self.connected = true;
        }
        Ok(())
    }

    pub fn get_tick_sub(&mut self) -> Subscription<MarketData> {
        self.subscription.as_mut().unwrap().write().unwrap().subscribe_with_filter(Box::new(|event| {
            match event {
                MarketData::Tick(_) => {
                    true
                },
                _ => {
                    false
                },
            }  
        }))
    }
    
    pub fn subscribe_kline(&mut self, symbol: &str, interval: &str) -> Receiver<MarketData> {
        let _ = self.server.subscribe_kline(symbol, interval);

        let (tx, rx) = channel::unbounded::<MarketData>();
        self.subscribers.push(Subscriber {
            symbol: symbol.to_string(),
            interval: interval.to_string(),
            sender: tx,
        });
        rx
    }

    pub fn subscribe_tick(&mut self, symbol: &str) -> Receiver<MarketData> {
        let _ = self.server.subscribe_tick(symbol);

        let (tx, rx) = channel::unbounded::<MarketData>();
        self.subscribers.push(Subscriber {
            symbol: symbol.to_string(),
            interval: "".to_string(),
            sender: tx,
        });
        rx
    }
  
    pub fn start(&self) -> JoinHandle<()> {
        let subscribers = self.subscribers.clone();
        let subscription_ref = self.subscription.as_ref().unwrap().clone();

        thread::spawn(move || {
            let mut should_break = false;
            let subscription = subscription_ref.read().unwrap();
            loop {
                let _ = subscription.recv(&mut |event| {
                    let subs_loop = & *subscribers; 
                    match event {
                        Some(data) => {
                            match data {
                                MarketData::Tick(t) => {
                                    for sub in subs_loop {
                                        if t.symbol == sub.symbol && sub.interval == "" {
                                            let _ = sub.sender.send(MarketData::Tick(t.clone()));
                                        }
                                    }
                                },
                                MarketData::Kline(k) => {
                                    for sub in subs_loop {
                                        if k.symbol == sub.symbol && k.interval == sub.interval {
                                            let _ = sub.sender.send(MarketData::Kline(k.clone()));
                                        }
                                    }
                                },
                                MarketData::MarketClosed => {
                                    for sub in subs_loop.iter() {
                                        let _ = sub.sender.send(MarketData::MarketClosed);
                                    }
                                    should_break = true;
                                },
                                _ => {},
                            }
                        },
                        None => {
                            should_break = true;
                        },
                    }
                });
                if should_break {
                    break;
                }
            }
        })
    }
}