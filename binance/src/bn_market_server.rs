use binance_future_connector::market::klines::KlineInterval;
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
use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use chrono::DateTime;
use crate::model::BinanceKline;

use super::model;

#[derive(Debug, Clone, Default)]
pub struct MarketTopic {
    pub symbol: String,
    pub interval: String,
}
pub struct BnMarketServer {
    pub config: model::Config,
    subscription: Arc<Mutex<Subscription<MarketData>>>,
    topics: Vec<MarketTopic>,
    handler: Option<Handler<()>>,
}

impl BnMarketServer {
    pub fn new(config: model::Config) -> Self {
        BnMarketServer {
            config: config,
            subscription: Arc::new(Mutex::new(Subscription::top())),
            topics: Vec::new(),
            handler: None,
        }
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
        };
        k
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
            };
            k_lines.push(k_line);
        }
        Ok(k_lines)
    }
}

impl MarketServer for BnMarketServer {
    fn connect(&mut self) -> Result<Subscription<MarketData>, AppError> {
        let outer_sucription = self.subscription.lock().unwrap().subscribe();
        Ok(outer_sucription)
    }

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

    fn start(&mut self) {
        let topics: Vec<MarketTopic> = self.topics.clone();
        let subscription_ref = self.subscription.clone();
        let depth_level = self.config.depth_level;
        let update_speed = self.config.tick_update_speed;

        let closure = move |rx: Rx<String>| {
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
                                println!("Invalid kline interval: {}", s);
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

            let subscription = subscription_ref.lock().unwrap();
            let mut last_tick = HashMap::<String, Tick>::new();
            let _ = keepalive.stream(&mut move |message| {
                let cmd = rx.try_recv();
                if cmd.is_ok() {
                    if cmd.unwrap() == "QUIT" {
                        return Ok(false);
                    }
                }

                let data = message.into_data();
                let string_data = String::from_utf8(data)?;

                let json_value: Value = serde_json::from_str(&string_data).unwrap();
                match json_value.get("e") {
                    Some(event_type) => {
                        if event_type.as_str().unwrap() == "depthUpdate" {
                            match serde_json::from_str::<model::BinanceDepthUpdate>(&string_data) {
                                Ok(depth) => {
                                    let value = last_tick.get_mut(&depth.symbol);
                                    if let Some(tick) = value {
                                        let mut t = tick.clone();
                                        t.asks = depth.asks;
                                        t.bids = depth.bids;
                                        subscription.send(&Some(MarketData::Tick(t)));
                                    }
                                },
                                _ => {},
                            }
                        } else if event_type.as_str().unwrap() == "kline" {
                            match serde_json::from_str::<model::BinanceKline>(&string_data) {
                                Ok(kline) => {
                                    if kline.kline_data.is_closed {
                                        let k = Self::convert_bn_kline(kline);
                                        subscription.send(&Some(MarketData::Kline(k)));
                                    }
                                },
                                _ => {},
                            }
                        } else if event_type.as_str().unwrap() == "24hrMiniTicker" {
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
                                        ..Default::default()
                                    };
                                    last_tick.insert(t.symbol.to_string(), t);
                                    //subscription.send(&Some(MarketData::Tick(t)));
                                },
                                Err(e) => {
                                    println!("{:?}", e);
                                },
                            }
                        }
                    },
                    None => {
                        println!("Received {}", string_data);
                    },
                }
                Ok(true)
            }, true);
        };

        self.handler = Some(InteractiveThread::spawn(closure));
    }

    fn close(self) {
        if let Some(h) = self.handler.as_ref() {
            let _ = h.sender.send("QUIT".to_string());
        }
    }
}