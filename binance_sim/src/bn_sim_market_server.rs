use binance::bn_market_server::{convert_json_to_k_lines, MarketTopic};
use binance::model;
use binance_future_connector::market::klines::KlineInterval;
use binance_future_connector::ureq::BinanceHttpClient;
use binance_future_connector::market as bn_market;

use common::error::AppError;
use log::info;
use market::market_server::{KLine, MarketData, MarketServer};
use common::msmc::*;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::model::SimMarketConfig;

pub struct BnSimMarketServer {
    config: SimMarketConfig,
    topics: Vec<MarketTopic>,
    subscription: Arc<Mutex<Subscription<MarketData>>>,
}

impl BnSimMarketServer {
    pub fn new(config: SimMarketConfig) -> Self {
        BnSimMarketServer {
            config: config,
            topics: Vec::new(),
            subscription: Arc::new(Mutex::new(Subscription::top())),
        }
    }
}

impl MarketServer for BnSimMarketServer {
    type Symbol = String;
    
    fn load_kline(&mut self, symbol: String, interval: &str, count: u32) -> Result<Vec<KLine>, AppError> {
        if self.config.start_time == 0 {
            return Err(AppError::new(-200, "The backtest start_time has not yet set."));
        }
        let client = BinanceHttpClient::default();
        let kline_interval = KlineInterval::from_str(interval).map_err(|e| {AppError::new(-200, &e)})?;
        let request = bn_market::klines(&symbol, kline_interval).limit(count).end_time(self.config.start_time);
        let data = model::get_resp_result(client.send(request), vec![])?;
        let klines = convert_json_to_k_lines(&symbol, interval, &data).map_err(|e| AppError::new(-200, format!("{:?}", e).as_str()))?;
        Ok(klines)
    }

    fn subscribe_tick(&mut self, _symbol: String) -> Result<(), AppError>{
        Err(AppError::new(-200, "subscribe_tick is not support in backtest mode"))
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
        self.topics.sort();
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
        thread::spawn(move|| {
            let mut temp = config.start_time;
            let mut kline_store: HashMap<String, (Vec<KLine>, usize)> = HashMap::new();
            while temp <= config.end_time {
                for topic in topics.iter() {
                    let ret = visit(&mut kline_store, topic.symbol.clone(), &topic.interval.clone(), temp);
                    
                    if let Ok(kline) = ret {
                        if let Some(v) = kline {
                            let subscrption = subscription_ref.lock().unwrap();
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

fn load_more(symbol: String, interval: &str, count: u32, start_time: u64) -> Result<Vec<KLine>, AppError> {
    let client = BinanceHttpClient::default();
    let kline_interval = KlineInterval::from_str(interval).map_err(|e| {AppError::new(-200, &e)})?;
    let request = bn_market::klines(&symbol, kline_interval).limit(count).start_time(start_time);
    let data = model::get_resp_result(client.send(request), vec![])?;
    let klines = convert_json_to_k_lines(&symbol, interval, &data).map_err(|e| AppError::new(-200, format!("{:?}", e).as_str()))?;
    Ok(klines)
}

fn visit(klines_store: &mut HashMap<String, (Vec<KLine>, usize)>, symbol: String, interval: &str, current_time: u64) -> Result<Option<KLine>, AppError> {
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
        let klines = load_more(symbol.clone(), interval, 500, current_time)?;
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