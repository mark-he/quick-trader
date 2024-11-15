use binance_future_connector::market::klines::KlineInterval;
use binance_future_connector::market_stream::enums::{Level, UpdateSpeed};
use binance_future_connector::market_stream::mini_ticker::MiniTickerStream;
use binance_future_connector::market_stream::partial_depth::PartialDepthStream;
use binance_future_connector::ureq::BinanceHttpClient;
use binance_future_connector::wss_keepalive::WssKeepalive;
use binance_future_connector::{config, market as bn_market, market_stream::kline::KlineStream,
};
use common::thread::{Handler, InteractiveThread, Rx};
use serde_json::Value;

use common::error::AppError;
use market::market_server::{KLine, MarketData, MarketServer, Tick};
use common::msmc::*;
use tungstenite::Message;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use chrono::DateTime;
use crate::model::BinanceKline;
use log::*;
use super::model;

#[derive(Debug, Clone, Default)]
pub struct MarketTopic {
    pub symbol: String,
    pub interval: String,
}

pub struct WssStream {
    subscription: Arc<Mutex<Subscription<MarketData>>>,
    handler: Option<Handler<()>>,
    connect_ticket: Arc<AtomicUsize>,
    server_ping: Arc<AtomicUsize>,
    depth_level: Level,
    update_speed: Option<UpdateSpeed>,
}

impl WssStream {
    pub fn new(depth_level: Level,  update_speed: Option<UpdateSpeed>,) -> Self {
        WssStream {
            subscription: Arc::new(Mutex::new(Subscription::top())),
            handler : None,
            connect_ticket: Arc::new(AtomicUsize::new(0)),
            server_ping: Arc::new(AtomicUsize::new(0)),
            depth_level,
            update_speed,
        }
    }

    pub fn cleanup(&mut self) {
        self.subscription = Arc::new(Mutex::new(Subscription::top()));
        self.server_ping = Arc::new(AtomicUsize::new(0));
        self.handler = None;
    }

    pub fn subscribe(&mut self) -> Subscription<MarketData> {
        self.subscription.lock().unwrap().subscribe()
    }

    pub fn connect(&mut self, topics: Vec<MarketTopic>) {
        let connect_ticket = self.connect_ticket.fetch_add(1, Ordering::SeqCst);
        let server_ping_ref = self.server_ping.clone();
        let connect_ticket_ref = self.connect_ticket.clone();
        let subscription_ref = self.subscription.clone();
        let depth_level = self.depth_level.clone();
        let update_speed = self.update_speed.clone();

        let closure = move |_rx: Rx<String>| {
            let subscription = subscription_ref.lock().unwrap();
            let mut keepalive: WssKeepalive = WssKeepalive::new(&config::wss_api()).prepare(move |conn| {
                let mut tick_set = HashSet::new();
                for topic in topics.iter() {
                    if topic.interval == "" {
                        if !tick_set.contains(topic.symbol.as_str()) {
                            tick_set.insert(topic.symbol.to_string());
                        }
                    } 
                }

                for topic in topics.iter() {
                    if topic.interval != "" {
                        let kline_interval_ret= KlineInterval::from_str(&topic.interval);
                        match kline_interval_ret {
                            Ok(interval)=> {
                                conn.subscribe(vec![
                                    &KlineStream::new(topic.symbol.as_str(), interval).into(),
                                ]);
                                tick_set.insert(topic.symbol.to_string());
                            },
                            Err(s) => {
                                error!("{}", &s);
                            },
                        }
                    }
                }

                for symbol in tick_set.iter() {
                    let partial_depth;
                    if let Some(speed) = update_speed {
                        partial_depth = PartialDepthStream::new(symbol, depth_level).update_speed(speed);
                    } else {
                        partial_depth = PartialDepthStream::new(symbol, depth_level);
                    }
                    conn.subscribe(vec![
                        &MiniTickerStream::from_symbol(symbol).into(),
                        &partial_depth.into(),
                    ]);
                }
            });
            
            let mut last_ticks = HashMap::<String, Tick>::new();
            let mut last_klines = HashMap::<String, KLine>::new();

            let _ = keepalive.stream(&mut move |message| {
                if connect_ticket != connect_ticket_ref.load(Ordering::SeqCst) - 1 {
                    return Ok(true);
                }
                match message {
                    Message::Text(string_data) => {
                        let json_value: Value = serde_json::from_str(&string_data).unwrap();
                        match json_value.get("e") {
                            Some(event_type) => {
                                debug!("Received event: {}", string_data);
                                let event = event_type.as_str().unwrap();
                                match event {
                                    "depthUpdate" => {
                                        match serde_json::from_str::<model::BinanceDepthUpdate>(&string_data) {
                                            Ok(depth) => {
                                                let value = last_ticks.get_mut(&depth.symbol);
                                                if let Some(tick) = value {
                                                    let mut t = tick.clone();
                                                    t.asks = depth.asks;
                                                    t.bids = depth.bids;
                                                    subscription.send(&Some(MarketData::Tick(t)));
                                                }
                                            },
                                            _ => {},
                                        }
                                    },
                                    "kline" => {
                                        match serde_json::from_str::<model::BinanceKline>(&string_data) {
                                            Ok(kline) => {
                                                if kline.kline_data.is_closed {
                                                    let k = Self::convert_bn_kline(kline);

                                                    let key = format!("{}_{}", k.symbol, k.interval);
                                                    let prev_kline = last_klines.get(&key);
                                                    if let Some(prev) = prev_kline {
                                                        if k.timestamp > prev.timestamp {
                                                            last_klines.insert(key, k.clone());
                                                        }
                                                    } else {
                                                        last_klines.insert(key, k.clone());
                                                    }
                                                    subscription.send(&Some(MarketData::Kline(k)));
                                                }
                                            },
                                            _ => {},
                                        }
                                    },
                                    "24hrMiniTicker" => {
                                        match serde_json::from_str::<model::BinanceTick>(&string_data) {
                                            Ok(tick) => {
                                                let datetime = DateTime::from_timestamp((tick.event_time/1000) as i64, 0).unwrap();
                                                let t = Tick {
                                                    symbol: tick.symbol.clone(),
                                                    datetime: datetime.format("%Y-%m-%d %H:%M:%S").to_string(),
                                                    trading_day: datetime.format("%Y-%m-%d").to_string(),
                                                    open: tick.open_price,
                                                    high: tick.high_price,
                                                    low: tick.low_price,
                                                    close: tick.close_price,
                                                    volume: tick.total_traded_base_asset_volume,
                                                    turnover: tick.total_traded_quote_asset_volume,
                                                    timestamp: tick.event_time,
                                                    ..Default::default()
                                                };

                                                let prev_tick = last_ticks.get(&t.symbol);
                                                if let Some(prev) = prev_tick {
                                                    if t.timestamp > prev.timestamp {
                                                        last_ticks.insert(t.symbol.to_string(), t);
                                                    }
                                                } else {
                                                    last_ticks.insert(t.symbol.to_string(), t);
                                                }
                                            },
                                            Err(e) => {
                                                error!("{:?}", e);
                                            },
                                        }
                                    },
                                    _ => {
                                        debug!("Received other event: {}", string_data);
                                    },
                                }
                            },
                            None => {
                                warn!("Received unknown event: {}", string_data);
                            },
                        }
                    },
                    Message::Ping(data) => {
                        let string_data = String::from_utf8(data)?;
                        server_ping_ref.store(string_data.parse::<usize>()?, Ordering::SeqCst);
                        info!("Market server ping: {}", string_data);
                    },
                    _ => {
                        warn!("Unexpected message: {:?}", message);
                    },
                }
                Ok(true)
            }, true);
        };

        self.handler = Some(InteractiveThread::spawn(closure));
    }

    fn convert_bn_kline(kline: BinanceKline) -> KLine {
        let datetime = DateTime::from_timestamp((kline.kline_data.start_time/1000) as i64, 0).unwrap();
        let k = KLine {
            symbol: kline.kline_data.symbol.clone(),
            interval: kline.kline_data.interval.clone(),
            datetime: datetime.format("%Y-%m-%d %H:%M:%S").to_string(),
            open: kline.kline_data.open_price,
            high: kline.kline_data.high_price,
            low: kline.kline_data.low_price,
            close: kline.kline_data.close_price,
            volume: kline.kline_data.number_of_trades as f64,
            turnover: kline.kline_data.quote_asset_volume,
            timestamp: kline.kline_data.start_time,
        };
        k
    }

    fn close(&self) {
        self.connect_ticket.fetch_add(1, Ordering::SeqCst);
    }
}

pub struct BnMarketServer {
    pub config: model::Config,
    pub wss_stream: WssStream,
    topics: Vec<MarketTopic>,
}

impl BnMarketServer {
    pub fn new(config: model::Config) -> Self {
        let depth_level = config.depth_level;
        let tick_update_speed = config.tick_update_speed;
        BnMarketServer {
            config: config,
            wss_stream: WssStream::new(depth_level, tick_update_speed),
            topics: Vec::new(),
        }
    }

    fn convert_json_to_k_lines(symbol: &str, interval: &str, json_str: &str) -> Result<Vec<KLine>, Box<dyn std::error::Error>> {
        let data: Vec<Vec<serde_json::Value>> = serde_json::from_str(json_str)?;
        let mut k_lines = Vec::new();
    
        for line in data {
            let datetime = DateTime::from_timestamp((line[0].as_u64().unwrap()/1000) as i64, 0).unwrap();
            let k_line = KLine {
                symbol: symbol.to_string(),
                interval: interval.to_string(),
                datetime: datetime.format("%Y-%m-%d %H:%M:%S").to_string(),
                open: line[1].as_str().unwrap().parse::<f64>()?,
                high: line[2].as_str().unwrap().parse::<f64>()?,
                low: line[3].as_str().unwrap().parse::<f64>()?,
                close: line[4].as_str().unwrap().parse::<f64>()?,
                volume: line[5].as_str().unwrap().parse::<f64>()?,
                turnover: line[7].as_str().unwrap().parse::<f64>()?,
                timestamp: line[0].as_u64().unwrap(),
            };
            k_lines.push(k_line);
        }
        Ok(k_lines)
    }
}

impl MarketServer for BnMarketServer {
    fn load_kline(&mut self, symbol: &str, interval: &str, count: u32) -> Result<Vec<KLine>, AppError> {
        let client = BinanceHttpClient::default();
        let kline_interval = KlineInterval::from_str(interval).map_err(|e| {AppError::new(-200, &e)})?;
        let request = bn_market::klines(symbol,kline_interval).limit(count);
        let data = model::get_resp_result(client.send(request), vec![])?;
        let klines = Self::convert_json_to_k_lines(symbol, interval, &data).map_err(|e| AppError::new(-200, format!("{:?}", e).as_str()))?;
        Ok(klines)
    }

    fn subscribe_tick(&mut self, symbol: &str) {
        let mut found = false;
        for topic in self.topics.iter() {
            if topic.symbol == symbol && topic.interval == "" {
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
            if topic.symbol == symbol {
                if topic.interval == interval {
                    found = true;
                    break;
                }
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

    fn get_server_ping(&self) -> usize {
        self.wss_stream.server_ping.load(Ordering::SeqCst)
    }

    fn init(&mut self) -> Result<(), AppError> {
        self.wss_stream.cleanup();
        Ok(())
    }

    fn start(&mut self) -> Result<Subscription<MarketData>, AppError> {
        let sub = self.wss_stream.subscribe();
        self.wss_stream.connect(self.topics.clone());
        Ok(sub)
    }

    fn close(&self) {
        self.wss_stream.close();
    }
}