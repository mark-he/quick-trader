use crate::market_server::KLine;

use super::market_server::{MarketData, MarketServer};
use common::{error::AppError, msmc::Subscription, thread::{Handler, InteractiveThread, Rx}};
use log::info;
use std::{sync::{atomic::{AtomicUsize, Ordering}, Arc, Mutex}, vec};
use crossbeam::channel::{self, Receiver, Sender};

#[derive(Clone, Debug)]
struct Subscriber {
    symbol: String,
    interval: String,
    sender: Sender<MarketData>,
}

pub struct MarketGateway<S: MarketServer> {
    server: S,
    subscription: Arc<Mutex<Subscription<MarketData>>>,
    subscribers : Vec<Subscriber>,
    pub handler: Option<Handler<()>>,
    start_ticket: Arc<AtomicUsize>,
}

impl <S: MarketServer> MarketGateway<S> {
    pub fn new(server: S) -> Self {
        MarketGateway {
            server,
            subscription: Arc::new(Mutex::new(Subscription::top())),
            subscribers: vec![],
            handler: None,
            start_ticket: Arc::new(AtomicUsize::new(0)),
        }
    }
}

impl<S: MarketServer> MarketGateway<S> {
    pub fn init(&mut self) -> Result<(), AppError> {
        self.server.init()
    }

    pub fn get_tick_sub(&mut self) -> Subscription<MarketData> {
        self.subscription.lock().unwrap().subscribe_with_filter(|event| {
            match event {
                MarketData::Tick(_) => {
                    true
                },
                _ => {
                    false
                },
            }  
        })
    }
    
    pub fn load_kline(&mut self, symbol: &str, interval: &str, count: u32) -> Result<Vec<KLine>, AppError> {
        self.server.load_kline(symbol, interval, count)
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
  
    pub fn start(&mut self) -> Result<(), AppError> {
        info!("gateway started");
        let start_ticket = self.start_ticket.fetch_add(1, Ordering::SeqCst);
        let start_ticket_ref = self.start_ticket.clone();
        let subscription = self.server.start()?;
        let subscribers = self.subscribers.clone();

        let closure = move |_rx: Rx<String>| {
            let mut continue_flag = true;
            let _ = subscription.stream(&mut |event| {
                if start_ticket != start_ticket_ref.load(Ordering::SeqCst) - 1 {
                    return Ok(false);
                }
                match event {
                    Some(data) => {
                        match data {
                            MarketData::Tick(t) => {
                                for sub in subscribers.iter() {
                                    if t.symbol == sub.symbol && sub.interval == "" {
                                        let _ = sub.sender.send(MarketData::Tick(t.clone()));
                                    }
                                }
                            },
                            MarketData::Kline(k) => {
                                for sub in subscribers.iter() {
                                    if k.symbol == sub.symbol && k.interval == sub.interval {
                                        let _ = sub.sender.send(MarketData::Kline(k.clone()));
                                    }
                                }
                            },
                            MarketData::MarketClosed => {
                                for sub in subscribers.iter() {
                                    let _ = sub.sender.send(MarketData::MarketClosed);
                                }
                                continue_flag = false;
                            },
                            _ => {},
                        }
                    },
                    None => {
                        
                    },
                }
                Ok(continue_flag)
            }, true);

        };
        self.handler = Some(InteractiveThread::spawn(closure));
        Ok(())
    }

    pub fn close(&self) {
        self.start_ticket.fetch_add(1, Ordering::SeqCst);
        self.server.close();
    }

    pub fn get_server_ping(&self) -> usize {
        self.server.get_server_ping()
    }
}