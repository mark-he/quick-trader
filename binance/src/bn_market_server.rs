use binance_spot_connector::market_stream::ticker::TickerStream;
use binance_spot_connector::tungstenite::WebSocketState;
use binance_spot_connector::{
    market::klines::KlineInterval, market_stream::kline::KlineStream,
    tungstenite::BinanceWebSocketClient,
};
use serde_json::Value;
use tungstenite::stream::MaybeTlsStream;

use common::error::AppError;
use market::market_server::{KLine, MarketData, MarketServer, Tick};
use std::net::TcpStream;
use std::sync::{Arc, Mutex, RwLock};
use std::thread::{self};
use common::msmc::*;
use std::collections::{HashMap, HashSet};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, NaiveDateTime};

const BINANCE_WSS_BASE_URL: &str = "wss://testnet.binance.vision/ws";

#[derive(Debug, Clone, Default)]
pub struct Config {
}
#[derive(Debug, Serialize, Deserialize)]
struct BinanceKline {
    /// Event type
    #[serde(rename = "e")]
    event_type: String,
    /// Event time
    #[serde(rename = "E")]
    event_time: u64,
    /// Symbol of the trading pair
    #[serde(rename = "s")]
    symbol: String,
    /// Kline data
    #[serde(rename = "k")]
    kline_data: KlineData,
}

#[derive(Debug, Serialize, Deserialize)]
struct KlineData {
    /// Start time of the kline
    #[serde(rename = "t")]
    start_time: u64,
    /// Close time of the kline
    #[serde(rename = "T")]
    close_time: u64,
    /// Symbol of the trading pair
    #[serde(rename = "s")]
    symbol: String,
    /// Interval
    #[serde(rename = "i")]
    interval: String,
    /// First trade ID
    #[serde(rename = "f")]
    first_trade_id: u64,
    /// Last trade ID
    #[serde(rename = "L")]
    last_trade_id: u64,
    /// Open price
    #[serde(rename = "o")]
    open_price: String,
    /// Close price
    #[serde(rename = "c")]
    close_price: String,
    /// High price
    #[serde(rename = "h")]
    high_price: String,
    /// Low price
    #[serde(rename = "l")]
    low_price: String,
    /// Volume of the base asset
    #[serde(rename = "v")]
    base_asset_volume: String,
    /// Number of trades
    #[serde(rename = "n")]
    number_of_trades: u64,
    /// Whether the kline is closed
    #[serde(rename = "x")]
    is_closed: bool,
    /// Volume of the quote asset
    #[serde(rename = "q")]
    quote_asset_volume: String,
    /// Taker buy volume of the base asset
    #[serde(rename = "V")]
    taker_buy_base_asset_volume: String,
    /// Taker buy volume of the quote asset
    #[serde(rename = "Q")]
    taker_buy_quote_asset_volume: String,
    /// Ignore
    #[serde(rename = "B")]
    ignored_value: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct BinanceTick {
    #[serde(rename = "e")]
    /// Event type
    event_type: String,
    #[serde(rename = "E")]
    /// Event time
    event_time: u64,
    #[serde(rename = "s")]
    /// Symbol of the trading pair
    symbol: String,
    #[serde(rename = "p")]
    /// Price change
    price_change: String,
    #[serde(rename = "P")]
    /// Percentage of price change
    price_change_percent: String,
    #[serde(rename = "w")]
    /// Weighted average price
    weighted_average_price: String,
    #[serde(rename = "x")]
    /// First trade price
    first_trade_price: String,
    #[serde(rename = "c")]
    /// Last price
    last_price: String,
    #[serde(rename = "Q")]
    /// Last quantity
    last_quantity: String,
    #[serde(rename = "b")]
    /// Best bid price
    best_bid_price: String,
    #[serde(rename = "B")]
    /// Best bid quantity
    best_bid_quantity: String,
    #[serde(rename = "a")]
    /// Best ask price
    best_ask_price: String,
    #[serde(rename = "A")]
    /// Best ask quantity
    best_ask_quantity: String,
    #[serde(rename = "o")]
    /// Open price
    open_price: String,
    #[serde(rename = "h")]
    /// High price
    high_price: String,
    #[serde(rename = "l")]
    /// Low price
    low_price: String,
    #[serde(rename = "v")]
    /// Total traded base asset volume
    total_traded_base_asset_volume: String,
    #[serde(rename = "q")]
    /// Total traded quote asset volume
    total_traded_quote_asset_volume: String,
    #[serde(rename = "O")]
    /// Statistics open time
    statistics_open_time: u64,
    #[serde(rename = "C")]
    /// Statistics close time
    statistics_close_time: u64,
    #[serde(rename = "F")]
    /// First trade ID
    first_trade_id: u64,
    #[serde(rename = "L")]
    /// Last trade ID
    last_trade_id: u64,
    #[serde(rename = "n")]
    /// Total number of trades
    total_number_of_trades: u64,
}

pub struct MarketTopic {
    pub symbol: String,
    pub interval: String,
}
pub struct BnMarketServer {
    conn: Option<Arc<Mutex<WebSocketState<MaybeTlsStream<TcpStream>>>>>,
    subscription: Arc<RwLock<Subscription<MarketData>>>,
    topics: Vec<MarketTopic>,
}

impl BnMarketServer {
    pub fn new() -> Self {
        BnMarketServer {
            conn: None,
            subscription: Arc::new(RwLock::new(Subscription::top())),
            topics: Vec::new(),
        }
    }

    fn interval_from_str(s: &str) -> Result<KlineInterval, String> {
        match s {
            "1m" => Ok(KlineInterval::Minutes1),
            "3m" => Ok(KlineInterval::Minutes3),
            "5m" => Ok(KlineInterval::Minutes5),
            "15m" => Ok(KlineInterval::Minutes15),
            "30m" => Ok(KlineInterval::Minutes30),
            "1h" => Ok(KlineInterval::Hours1),
            "2h" => Ok(KlineInterval::Hours2),
            "4h" => Ok(KlineInterval::Hours4),
            "6h" => Ok(KlineInterval::Hours6),
            "8h" => Ok(KlineInterval::Hours8),
            "12h" => Ok(KlineInterval::Hours12),
            "1d" => Ok(KlineInterval::Days1),
            "3d" => Ok(KlineInterval::Days3),
            "1w" => Ok(KlineInterval::Weeks1),
            "1M" => Ok(KlineInterval::Months1),
            _ => Err(format!("Invalid duration: {}", s)),
        }
    }
}

impl MarketServer for BnMarketServer {
    fn connect(&mut self, _prop : &HashMap<String, String>) -> Result<Subscription<MarketData>, AppError> {
        let ret = BinanceWebSocketClient::connect_with_url(BINANCE_WSS_BASE_URL);
        match ret {
            Ok(conn) => {
                self.conn = Some(Arc::new(Mutex::new(conn)));
                let outer_sucription = self.subscription.write().unwrap().subscribe();
                Ok(outer_sucription)
            },
            Err(_) => {
                Err(AppError::new(-200, "Closed connection of market server"))
            },
        }
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

    fn start(&mut self) -> Result<(), AppError> {
        let conn_ref = self.conn.as_mut().unwrap();

        let mut tick_set = HashSet::new();
        for topic in self.topics.iter() {
            if topic.interval == "" {
                if !tick_set.contains(topic.symbol.as_str()) {
                    conn_ref.lock().unwrap().subscribe(vec![
                        &TickerStream::from_symbol(topic.symbol.as_str()).into(),
                    ]);
                    tick_set.insert(topic.symbol.to_string());
                }
            } 
        }
        for topic in self.topics.iter() {
            if topic.interval != "" {
                let kline_interval_ret= BnMarketServer::interval_from_str(topic.interval.as_str());
                match kline_interval_ret {
                    Ok(interval)=> {
                        conn_ref.lock().unwrap().subscribe(vec![
                            &KlineStream::new(topic.symbol.as_str(), interval).into(),
                        ]);
                        if !tick_set.contains(topic.symbol.as_str()) {
                            conn_ref.lock().unwrap().subscribe(vec![
                                &TickerStream::from_symbol(topic.symbol.as_str()).into(),
                            ]);
                            tick_set.insert(topic.symbol.to_string());
                        }
                    },
                    Err(s) => {
                        return Err(AppError::new(-200, s.as_str()))
                    },
                }
            }
        }

        let conn_ref = self.conn.as_ref().unwrap().clone();
        let subscription_ref = self.subscription.clone();

        thread::spawn(move || {
            let mut conn = conn_ref.lock().unwrap();
            let subscription = subscription_ref.read().unwrap();

            while let Ok(message) = conn.as_mut().read() {
                let data = message.into_data();
                let string_data = String::from_utf8(data).expect("Found invalid UTF-8 chars");
                let json_value: Value = serde_json::from_str(&string_data).unwrap();
            
                match json_value.get("e") {
                    Some(event_type) => {
                        if event_type.as_str().unwrap() == "kline" {
                            match serde_json::from_str::<BinanceKline>(&string_data) {
                                Ok(kline) => {
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
                                },
                                _ => {},
                            }
                        } else {
                            match serde_json::from_str::<BinanceTick>(&string_data) {
                                Ok(tick) => {
                                    let datetime = DateTime::from_timestamp((tick.statistics_open_time/1000) as i64, 0).unwrap();
                                    let t = Tick {
                                        symbol: tick.symbol.clone(),
                                        datetime: datetime.format("%Y-%m-%d %H:%M:%S").to_string(),
                                        trading_day: datetime.format("%Y-%m-%d").to_string(),
                                        open: tick.open_price.parse::<f64>().unwrap(),
                                        high: tick.high_price.parse::<f64>().unwrap(),
                                        low: tick.low_price.parse::<f64>().unwrap(),
                                        close: tick.last_price.parse::<f64>().unwrap(),
                                        volume: tick.total_number_of_trades as i32,
                                        turnover: tick.total_traded_quote_asset_volume.parse::<f64>().unwrap(),
                                        open_interest: tick.last_quantity.parse::<f64>().unwrap(),
                                        ..Default::default()
                                    };
                                    subscription.send(&Some(MarketData::Tick(t)));
                                },
                                _ => {},
                            }
                        }
                    },
                    _ => {
                        println!("Skipping this data because it is not kline or ticker.");
                    },
                }
            }
        });
        Ok(())
    }

    fn close(&mut self) {
        if let Some(arc_mutex_conn) = &mut self.conn {
            let mut conn_guard = arc_mutex_conn.lock().unwrap();
            let _ = conn_guard.close();
        }
    }
}