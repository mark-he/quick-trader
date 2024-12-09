use bybit_connector::enums::{Category, KlineInterval};
use bybit_connector::market_stream::depth::DepthStream;
use bybit_connector::market_stream::ticker::TickerStream;
use bybit_connector::ureq::BybitHttpClient;
use bybit_connector::wss_keepalive::WssKeepalive;
use bybit_connector::{config, market as bb_market, market_stream::kline::KlineStream,
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
use std::vec;
use chrono::DateTime;
use log::*;

use crate::model::{self, BbMarketConfig, KlineDetail, KlineQueryResp, ServerTime};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct MarketTopic {
    pub symbol: String,
    pub interval: String,
}

impl PartialOrd for MarketTopic {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let self_interval = KlineInterval::from_str(&self.interval);
        let other_interval = KlineInterval::from_str(&other.interval);
        let ret =  self_interval.cmp(&other_interval);
        return Some(ret);
    }
}

impl Ord for MarketTopic {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_interval = KlineInterval::from_str(&self.interval);
        let other_interval = KlineInterval::from_str(&other.interval);
        let ret =  self_interval.cmp(&other_interval);
        return  ret;
    }
}

pub struct WssStream {
    subscription: Arc<Mutex<Subscription<MarketData>>>,
    handler: Option<Handler<()>>,
    connect_ticket: Arc<AtomicUsize>,
    server_ping: Arc<AtomicUsize>,
    depth_level: u32,
}

impl WssStream {
    pub fn new(depth_level: u32) -> Self {
        WssStream {
            subscription: Arc::new(Mutex::new(Subscription::top())),
            handler : None,
            connect_ticket: Arc::new(AtomicUsize::new(0)),
            server_ping: Arc::new(AtomicUsize::new(0)),
            depth_level,
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
        let closure = move |_rx: Rx<String>| {
            let subscription = subscription_ref.lock().unwrap();
            let mut keepalive: WssKeepalive = WssKeepalive::new(&format!("{}/v5/public/linear", &config::wss_api())).prepare(move |conn| {
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
                    conn.subscribe(vec![
                        &TickerStream::new(symbol).into(),
                        &DepthStream::new(symbol, depth_level).into(),
                    ]);
                }
            });
            
            let mut last_ticks = HashMap::<String, Tick>::new();
            let mut last_klines = HashMap::<String, KLine>::new();

            let mut asks: Vec<Vec<f64>> = vec![];
            let mut bids: Vec<Vec<f64>> = vec![];
            let _ = keepalive.stream(&mut move |message| {
                if connect_ticket != connect_ticket_ref.load(Ordering::SeqCst) - 1 {
                    return Ok(true);
                }
                match message {
                    Message::Text(string_data) => {
                        let json_value: Value = serde_json::from_str(&string_data).unwrap();
                        let topic = json_value.get("topic");
                        let _type = json_value.get("type");
                        let data = json_value.get("data");
                        let ts = json_value.get("ts");
                        if let Some(topic_value) = topic {
                            let vs: Vec<&str> = topic_value.as_str().unwrap().split('.').collect();
                            let event = vs[0];
                            match event {
                                "orderbook" => {
                                    match serde_json::from_str::<model::BybitOrderbook>(&string_data) {
                                        Ok(depth) => {
                                            let value = last_ticks.get_mut(vs[2]);
                                            if let Some(tick) = value {
                                                let mut t = tick.clone();
                                                if depth.data.asks.len() > 0 {
                                                    asks = depth.data.asks;
                                                }
                                                if depth.data.bids.len() > 0 {
                                                    bids = depth.data.bids;
                                                }
                                                t.asks = asks.clone();
                                                t.bids = bids.clone();
                                                subscription.send(&MarketData::Tick(t));
                                            }
                                        },
                                        _ => {},
                                    }
                                },
                                "kline" => {
                                    match serde_json::from_str::<model::BybitKline>(&string_data) {
                                        Ok(kline) => {
                                            if kline.data[0].confirm {
                                                let k = convert_bb_kline(vs[2], &kline.data[0]);
                                                let key = format!("{}_{}", k.symbol, k.interval);
                                                let prev_kline = last_klines.get(&key);
                                                if let Some(prev) = prev_kline {
                                                    if k.timestamp > prev.timestamp {
                                                        last_klines.insert(key, k.clone());
                                                    }
                                                } else {
                                                    last_klines.insert(key, k.clone());
                                                }
                                                subscription.send(&MarketData::Kline(k));
                                            }
                                        },
                                        _ => {},
                                    }
                                },
                                "tickers" => {
                                    let _type = _type.unwrap().as_str().unwrap();
                                    match _type {
                                        "snapshot" => {
                                            match serde_json::from_str::<model::BybitTicker>(&string_data) {
                                                Ok(tick) => {
                                                    let datetime = DateTime::from_timestamp((tick.timestamp/1000) as i64, 0).unwrap();
                                                    let t = Tick {
                                                        symbol: tick.data.symbol.clone(),
                                                        datetime: datetime.format("%Y-%m-%d %H:%M:%S").to_string(),
                                                        open: tick.data.prev_price24h,
                                                        high: tick.data.high_price24h,
                                                        low: tick.data.low_price24h,
                                                        close: tick.data.last_price,
                                                        volume: tick.data.volume24h,
                                                        turnover: tick.data.turnover24h,
                                                        timestamp: tick.timestamp as u64,
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
                                        "delta" => {
                                            match data.unwrap() {
                                                Value::Object(map) => {
                                                    let prev_tick = last_ticks.get_mut(map.get("symbol").unwrap().as_str().unwrap());
                                                    if let Some(prev) = prev_tick {
                                                        if let Some(value) = ts {
                                                            let datetime = DateTime::from_timestamp((value.as_i64().unwrap()/1000) as i64, 0).unwrap();
                                                            prev.datetime = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
                                                        }
                                                        if let Some(value) = map.get("prevPrice24h") {
                                                            if let Ok(open) = value.as_str().unwrap().parse::<f64>() {
                                                                prev.open = open;
                                                            }
                                                        }
                                                        if let Some(value) = map.get("highPrice24h") {
                                                            if let Ok(high) = value.as_str().unwrap().parse::<f64>() {
                                                                prev.high = high;
                                                            }
                                                        }
                                                        if let Some(value) = map.get("lowPrice24h") {
                                                            if let Ok(low) = value.as_str().unwrap().parse::<f64>() {
                                                                prev.low = low;
                                                            }
                                                        }
                                                        if let Some(value) = map.get("lastPrice") {
                                                            if let Ok(close) = value.as_str().unwrap().parse::<f64>() {
                                                                prev.close = close;
                                                            }
                                                        }
                                                        if let Some(value) = map.get("volume24h") {
                                                            if let Ok(volume) = value.as_str().unwrap().parse::<f64>() {
                                                                prev.volume = volume;
                                                            }
                                                        }
                                                        if let Some(value) = map.get("turnover24h") {
                                                            if let Ok(turnover) = value.as_str().unwrap().parse::<f64>() {
                                                                prev.turnover = turnover;
                                                            }
                                                        }
                                                        if let Some(value) = ts {
                                                            prev.timestamp = value.as_u64().unwrap();
                                                        }
                                                    }
                                                },
                                                _ => {
                                                    debug!("Received non-json event: {}", string_data);
                                                },
                                            }
                                        },
                                        _ => {
                                            debug!("Received unknown event: {}", string_data);
                                        }
                                    }
                                },
                                _ => {
                                    debug!("Received other event: {}", string_data);
                                },
                            }
                        } else {
                            warn!("Received unknown event: {}", string_data);
                        }
                    }
                    ,
                    Message::Ping(data) => {
                        let string_data = String::from_utf8(data)?;
                        server_ping_ref.store(string_data.parse::<usize>()?, Ordering::SeqCst);
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

    fn close(&self) {
        self.connect_ticket.fetch_add(1, Ordering::SeqCst);
    }
}

pub type BbMarketServerType = dyn MarketServer<Symbol = String>;

pub struct BbMarketServer {
    pub config: BbMarketConfig,
    pub wss_stream: WssStream,
    topics: Vec<MarketTopic>,
}

impl BbMarketServer {
    pub fn new(config: BbMarketConfig) -> Self {
        let depth_level = config.depth_level;
        BbMarketServer {
            config: config,
            wss_stream: WssStream::new(depth_level),
            topics: Vec::new(),
        }
    }

    pub fn get_server_timestamp(&self) -> Result<u64, AppError> {
        let client = BybitHttpClient::default();
        let request = bb_market::time();
        let data = model::get_resp_result::<ServerTime>(client.send(request), vec![], false)?;

        if let Some(time) = data {
            return Ok(time.time_nano as u64)   
        }
        Err(AppError::new(-200, "Can not get servertime"))
    }
}

impl MarketServer for BbMarketServer {
    type Symbol = String;
    fn load_kline(&mut self, symbol: String, interval: &str, count: u32) -> Result<Vec<KLine>, AppError> {
        let client = BybitHttpClient::default();
        let kline_interval = KlineInterval::from_str(interval).map_err(|e| {AppError::new(-200, &e)})?;
        let request = bb_market::klines(Category::Linear, &symbol, kline_interval).limit(count as u64);

        let data = model::get_resp_result::<KlineQueryResp>(client.send(request), vec![], false)?;
        if let Some(kline_resp) = data {
            let klines = convert_json_to_k_lines(&symbol, interval, kline_resp).map_err(|e| AppError::new(-200, format!("{:?}", e).as_str()))?;
            Ok(klines)
        } else {
            Ok(vec![])
        }
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


pub fn convert_bb_kline(symbol: &str, kline: &KlineDetail) -> KLine {
    let datetime = DateTime::from_timestamp((kline.start/1000) as i64, 0).unwrap();
    let k = KLine {
        symbol: symbol.to_string(),
        interval: kline.interval.clone(),
        datetime: datetime.format("%Y-%m-%d %H:%M:%S").to_string(),
        open: kline.open,
        high: kline.high,
        low: kline.low,
        close: kline.close,
        volume: kline.volume,
        turnover: kline.turnover,
        taker_buy_volume: 0 as f64,
        taker_buy_turnover: 0 as f64,
        timestamp: kline.end as u64,
    };
    k
}

pub fn convert_json_to_k_lines(symbol: &str, interval: &str, kline_resp: KlineQueryResp) -> Result<Vec<KLine>, Box<dyn std::error::Error>> {
    let mut k_lines = Vec::new();

    for line in kline_resp.list {
        let datetime = DateTime::from_timestamp((line[0].as_str().parse::<i64>()?/1000) as i64, 0).unwrap();

        let k_line = KLine {
            symbol: symbol.to_string(),
            interval: interval.to_string(),
            datetime: datetime.format("%Y-%m-%d %H:%M:%S").to_string(),
            open: line[1].as_str().parse::<f64>()?,
            high: line[2].as_str().parse::<f64>()?,
            low: line[3].as_str().parse::<f64>()?,
            close: line[4].as_str().parse::<f64>()?,
            volume: line[5].as_str().parse::<f64>()?,
            turnover: line[6].as_str().parse::<f64>()?,
            taker_buy_volume: 0 as f64,
            taker_buy_turnover: 0 as f64,
            timestamp: line[0].as_str().parse::<u64>()?,
        };
        k_lines.push(k_line);
    }
    Ok(k_lines)
}

