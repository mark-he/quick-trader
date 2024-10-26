use binance_future_connector::market_stream::mini_ticker::MiniTickerStream;
use binance_future_connector::wss_keepalive::WssKeepalive;
use binance_future_connector::{config, market_stream::kline::KlineStream, market::klines::interval_from_str,
};
use common::thread::{Handler, InteractiveThread, Rx};
use serde_json::Value;

use common::error::AppError;
use market::market_server::{KLine, MarketData, MarketServer, Tick};
use common::msmc::*;
use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use chrono::DateTime;
use super::model;

#[derive(Debug, Clone, Default)]
pub struct Config {
}

#[derive(Debug, Clone, Default)]
pub struct MarketTopic {
    pub symbol: String,
    pub interval: String,
}
pub struct BnMarketServer {
    subscription: Arc<Mutex<Subscription<MarketData>>>,
    topics: Vec<MarketTopic>,
    handler: Option<Handler<()>>,
}

impl BnMarketServer {
    pub fn new() -> Self {
        BnMarketServer {
            subscription: Arc::new(Mutex::new(Subscription::top())),
            topics: Vec::new(),
            handler: None,
        }
    }
}

impl MarketServer for BnMarketServer {
    fn connect(&mut self) -> Result<Subscription<MarketData>, AppError> {
        let outer_sucription = self.subscription.lock().unwrap().subscribe();
        Ok(outer_sucription)
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

        let closure = move |rx: Rx<String>| {
            let mut wss = WssKeepalive::new(config::WSS_API).prepare(move |conn| {
                let mut tick_set = HashSet::new();
                for topic in topics.iter() {
                    if topic.interval == "" {
                        if !tick_set.contains(topic.symbol.as_str()) {
                            conn.subscribe(vec![
                                &MiniTickerStream::from_symbol(topic.symbol.as_str()).into(),
                            ]);
                            tick_set.insert(topic.symbol.to_string());
                        }
                    } 
                }
    
                for topic in topics.iter() {
                    if topic.interval != "" {
                        let kline_interval_ret= interval_from_str(topic.interval.as_str());
                        match kline_interval_ret {
                            Ok(interval)=> {
                                conn.subscribe(vec![
                                    &KlineStream::new(topic.symbol.as_str(), interval).into(),
                                ]);
                                if !tick_set.contains(topic.symbol.as_str()) {
                                    conn.subscribe(vec![
                                        &MiniTickerStream::from_symbol(topic.symbol.as_str()).into(),
                                    ]);
                                    tick_set.insert(topic.symbol.to_string());
                                }
                            },
                            Err(s) => {
                                println!("Invalid kline interval: {}", s);
                            },
                        }
                    }
                }
            });

            let subscription = subscription_ref.lock().unwrap();
            let _ = wss.stream(|message| {
                let cmd = rx.try_recv();
                if cmd.is_ok() {
                    if cmd.unwrap() == "QUIT" {
                        return Ok(false);
                    }
                }

                let data = message.into_data();
                let string_data = String::from_utf8(data).expect("Found invalid UTF-8 chars");

                let json_value: Value = serde_json::from_str(&string_data).unwrap();
                match json_value.get("e") {
                    Some(event_type) => {
                        if event_type.as_str().unwrap() == "kline" {
                            match serde_json::from_str::<model::BinanceKline>(&string_data) {
                                Ok(kline) => {
                                    if kline.kline_data.is_closed {
                                        let datetime = DateTime::from_timestamp((kline.kline_data.start_time/1000) as i64, 0).unwrap();
                                        let k = KLine {
                                            symbol: kline.kline_data.symbol.clone(),
                                            interval: kline.kline_data.interval.clone(),
                                            datetime: datetime.format("%Y-%m-%d %H:%M:%S").to_string(),
                                            open: kline.kline_data.open_price.parse::<f64>().unwrap(),
                                            high: kline.kline_data.high_price.parse::<f64>().unwrap(),
                                            low: kline.kline_data.low_price.parse::<f64>().unwrap(),
                                            close: kline.kline_data.close_price.parse::<f64>().unwrap(),
                                            volume: kline.kline_data.number_of_trades as i32,
                                            turnover: kline.kline_data.quote_asset_volume.parse::<f64>().unwrap(),
                                        };
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
                                        open: tick.open_price.parse::<f64>().unwrap(),
                                        high: tick.high_price.parse::<f64>().unwrap(),
                                        low: tick.low_price.parse::<f64>().unwrap(),
                                        close: tick.close_price.parse::<f64>().unwrap(),
                                        volume: tick.total_traded_base_asset_volume.parse::<f64>().unwrap(),
                                        turnover: tick.total_traded_quote_asset_volume.parse::<f64>().unwrap(),
                                        open_interest: 0 as f64,
                                        ..Default::default()
                                    };
                                    subscription.send(&Some(MarketData::Tick(t)));
                                },
                                Err(e) => {
                                    println!("{:?}", e);
                                },
                            }
                        }
                    },
                    None => {},
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