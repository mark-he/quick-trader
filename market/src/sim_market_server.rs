

use common::error::AppError;
use serde::{Deserialize, Serialize};
use crate::market_server::{KLine, MarketData, MarketServer, Tick};
use common::msmc::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize,)]
pub struct SimMarketConfig {
    pub start_time: u64,
    pub end_time: u64,
    pub interval: u64,
    pub lines_per_sec: u64,
}

#[derive(Debug, Clone, Default)]
pub struct MarketTopic {
    pub symbol: String,
    pub interval: String,
}

pub trait KLineLoader:Send + Sync {
    fn load_kline(&self, symbol: &str, interval: &str, count: u32, start_time: Option<u64>, end_time: Option<u64>) -> Result<Vec<KLine>, AppError>;
}

pub struct SimMarketServer {
    config: SimMarketConfig,
    topics: Vec<MarketTopic>,
    subscription: Arc<Mutex<Subscription<MarketData>>>,
    kline_loader: Option<Box<dyn KLineLoader>>,
}

impl SimMarketServer {
    pub fn new(config: SimMarketConfig, kline_loader: Box<dyn KLineLoader>) -> Self {
        SimMarketServer {
            config,
            topics: Vec::new(),
            subscription: Arc::new(Mutex::new(Subscription::top())),
            kline_loader: Some(kline_loader),
        }
    }
}

impl MarketServer for SimMarketServer {
    type Symbol = String;
    
    fn load_kline(&mut self, symbol: String, interval: &str, count: u32) -> Result<Vec<KLine>, AppError> {
        if self.config.start_time == 0 {
            return Err(AppError::new(-200, "The backtest start_time has not yet set."));
        }
        let klines = self.kline_loader.as_ref().unwrap().load_kline(&symbol, interval, count, None, Some(self.config.start_time))?;
        Ok(klines)
    }

    fn subscribe_tick(&mut self, symbol: String) -> Result<(), AppError>{
        let mut found = false;
        for topic in self.topics.iter() {
            if topic.symbol == symbol && topic.interval == "" {
                found = true;
                break;
            }
        }

        if !found {
            let topic = MarketTopic {
                symbol: symbol,
                interval: "".to_string(),
            };
            self.topics.push(topic);
        }
        Ok(())
    }

    fn subscribe_kline(&mut self, symbol: String, interval: &str) -> Result<(), AppError>{
        let mut found = false;
        for topic in self.topics.iter() {
            if topic.symbol == symbol {
                if topic.interval == interval {
                    found = true;
                    break;
                }
            }
        }
        if !found {
            let topic = MarketTopic {
                symbol: symbol,
                interval: interval.to_string(),
            };
            self.topics.push(topic);
        }
        Ok(())
    }

    fn get_server_ping(&self) -> usize {
        return 0;
    }

    fn init(&mut self) -> Result<(), AppError> {
        Ok(())
    }

    fn start(&mut self) -> Result<Subscription<MarketData>, AppError> {
        let sub = self.subscription.lock().unwrap().subscribe();

        let config = self.config.clone();
        let topics = self.topics.clone();
        let subscription_ref = self.subscription.clone();
        let kline_loader = self.kline_loader.take().unwrap();
        thread::spawn(move|| {
            let mut temp = config.start_time;
            let mut kline_store: HashMap<String, (Vec<KLine>, usize)> = HashMap::new();
            while temp <= config.end_time {
                for topic in topics.iter() {
                    let ret = visit(&mut kline_store, topic.symbol.clone(), &topic.interval.clone(), temp, &kline_loader);
                    if let Ok(kline) = ret {
                        if let Some(v) = kline {
                            let subscrption = subscription_ref.lock().unwrap();
                            let tick = Tick {
                                symbol: v.symbol.clone(),
                                datetime: v.datetime.clone(),
                                open: 0 as f64,
                                high: 0 as f64,
                                low: 0 as f64,
                                close: v.close,
                                volume: 0 as f64,
                                turnover: 0 as f64,
                                bids: vec![],
                                asks: vec![],
                                timestamp: v.timestamp,
                            };
                            subscrption.send(&MarketData::Tick(tick));
                            subscrption.send(&MarketData::Kline(v));
                        }
                    } else {
                        panic!("Error when running bn_sim_market_server");
                    }
                }
                temp = temp + config.interval;
                if config.lines_per_sec > 0 && 1000 / config.lines_per_sec > 0 {
                    thread::sleep(Duration::from_millis(1000 / config.lines_per_sec));
                }
            }
        });
        Ok(sub)
    }

    fn close(&self) {
    }
}


fn visit(klines_store: &mut HashMap<String, (Vec<KLine>, usize)>, symbol: String, interval: &str, current_time: u64, kline_loader: &Box<dyn KLineLoader>) -> Result<Option<KLine>, AppError> {
    let item = klines_store.get_mut(&symbol);
    let mut need_more = true;
    if let Some(v) = item {
        if v.0.len() > 0 {
            need_more = v.1 >= v.0.len();
        } else {
            need_more = false;
        }
    }
    if need_more {
        let klines = kline_loader.load_kline(&symbol, interval, 500, Some(current_time), None)?;
        klines_store.insert(symbol.clone(), (klines, 0));
    }

    let item = klines_store.get_mut(&symbol);

    if let Some(v) = item {
        if v.0.len() > 0 && v.1 < v.0.len() {
            let kline = v.0.get(v.1);
            if let Some(value) = kline {
                if value.timestamp <= current_time {
                    v.1 = v.1 + 1;
                    return Ok(Some(value.clone()));
                }
            }
        }
    }
    Ok(None)
}