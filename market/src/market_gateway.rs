use super::{kline::KLine, market_server::{MarketData, MarketServer}};
use std::{collections::HashMap, thread::{self, JoinHandle}};
use common::{error::AppError, msmc::Subscription};
use std::sync::{Arc, Mutex, RwLock};
use crossbeam::channel::{self, Receiver, Sender};
use super::kline::KLineCombiner;

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
                subscribers: Arc::new(RwLock::new(Vec::new())),
                subscribed_symbols: Arc::new(RwLock::new(vec![])),
            })));
        }
    }

    pub fn get_gateway() -> SharedGw {
        unsafe {
            INSTANCE.as_ref().unwrap().clone()
        }
    }
}

pub struct MarketGateway {
    server: Box<dyn MarketServer>,
    connected: bool,
    subscription: Option<Arc<RwLock<Subscription<MarketData>>>>,
    subscribers : Arc<RwLock<Vec<(String, Sender<MarketData>, Option<KLineCombiner>)>>>,
    subscribed_symbols : Arc<RwLock<Vec<String>>>,
}

impl MarketGateway {
    pub fn connect(&mut self, prop : &HashMap<String, String>)  -> Result<(), AppError> {
        if !self.connected {
            self.subscription = Some(Arc::new(RwLock::new(self.server.connect(prop)?)));
            self.connected = true;
        }
        Ok(())
    }

    pub fn get_market_sub(&mut self) -> Subscription<MarketData> {
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

    pub fn subscribe(&mut self, symbol: &str, kline_combiner: Option<KLineCombiner>) -> Result<Receiver<MarketData>, AppError> {
        let symbols = &mut *self.subscribed_symbols.write().unwrap();
        let mut found = false;
        for s in symbols.iter() {
            if s == symbol {
                found = true;
                break;
            }
        }
        if !found {
            let _ = self.server.subscribe_tick(symbol)?;
            symbols.push(symbol.to_string());
        }

        let (tx, rx) = channel::unbounded::<MarketData>();
        let subscribers= &mut *self.subscribers.write().unwrap();
        subscribers.push((symbol.to_string(), tx, kline_combiner));
        Ok(rx)
    }
  
    pub fn start(&self) -> JoinHandle<()> {
        let subscribers_ref = self.subscribers.clone();
        let subscription_ref = self.subscription.as_ref().unwrap().clone();
        thread::spawn(move || {
            let mut should_break = false;
            let subscribers = &mut *subscribers_ref.write().unwrap();
            let subscription = subscription_ref.read().unwrap();
            loop {
                let _ = subscription.recv(&mut |event| {
                    let subs_loop = &mut *subscribers; 
                    match event {
                        Some(data) => {
                            match data {
                                MarketData::Tick(t) => {
                                    for sub in subs_loop {
                                        if t.symbol == sub.0 {
                                            let mut found_tick = false;
                                            if let Some(combiner) = &mut sub.2 {
                                                let kline = KLine {
                                                    symbol: t.symbol.clone(),
                                                    datetime: t.datetime.clone(),
                                                    open: t.open,
                                                    high: t.high,
                                                    low: t.low,
                                                    close: t.close,
                                                    volume: t.volume,
                                                    turnover: t.turnover,
                                                };
                                                let mut new_kline = combiner.combine_tick(&kline, true);
                                                if let Some(kline) = new_kline.take() {
                                                    let _ = sub.1.send(MarketData::Kline(kline));
                                                }
                                            } else {
                                                found_tick = true;
                                            }
                                            if found_tick {
                                                let _ = sub.1.send(MarketData::Tick(t.clone()));
                                            }
                                        }
                                    }
                                },
                                MarketData::MarketClosed => {
                                    for sub in subs_loop.iter() {
                                        let _ = sub.1.send(MarketData::MarketClosed);
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