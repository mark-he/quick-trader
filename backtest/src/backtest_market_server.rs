use common::thread::{Handler, InteractiveThread, Rx};
use market::kline::KLineCombiner;
use market::market_server::{KLine, MarketData, MarketServer, Tick};
use common::{error::AppError, msmc::*};
use std::collections::HashMap;
use std::fs::File;
use std::error::Error;
use std::thread::{self};
use std::time::Duration;
use std::collections::HashSet;

static FILE_PATH :&str = "d:";

#[derive(Clone)]
pub struct MarketTopic {
    pub symbol: String,
    pub interval: String,
}
pub struct BacktestMarketServer {
    connected: bool,
    subscription:Subscription<MarketData>,
    topics: Vec<MarketTopic>,
}

impl BacktestMarketServer {
    pub fn new() -> Self {
        BacktestMarketServer {
            connected: false,
            subscription: Subscription::top(),
            topics: Vec::new(),
        }
    }
}

impl MarketServer for BacktestMarketServer {
    fn connect(&mut self, _prop : &HashMap<String, String>) -> Result<Subscription<MarketData>, AppError> {
        if !self.connected {
            let sub = self.subscription.subscribe();
            self.connected = true;
            Ok(sub)
        } else {
            Err(AppError::new(-100, "Duplicated connected to MarketServer"))
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
            if topic.symbol == symbol && topic.interval == interval {
                found = true;
                break;
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

    fn start(self)  -> Handler<()> {
        let closure = move |_: Rx<String>| {
            let mut symbol_set: HashSet<String> = HashSet::new();
            for topic in self.topics.iter() {
                symbol_set.insert(topic.symbol.clone());
            }
            for symbol in symbol_set {
                let result = read_csv_file(String::from(format!("{}/{}.csv", FILE_PATH, symbol)));
                let mut combiner_map:HashMap<String, KLineCombiner> = HashMap::new();
                if let Ok(data) = result {
                    for o in data.iter() {
                        thread::sleep(Duration::from_millis(500));
                        let result = _convert_tick(symbol.as_str(), &o);
                        if let Ok(t) = result {
                            let _ = self.subscription.send(&Some(MarketData::Tick(t.clone())));
                            for topic in self.topics.iter() {
                                if topic.symbol == symbol && topic.interval != "" {
                                    let combiner = combiner_map.entry(format!("{}_{}", topic.symbol, topic.interval)).or_insert(KLineCombiner::new(topic.interval.as_str(), 100, Some(21)));
                                    let kline = KLine {
                                        symbol: t.symbol.clone(),
                                        datetime: t.datetime.clone(),
                                        interval: topic.interval.clone(),
                                        open: t.open,
                                        high: t.high,
                                        low: t.low,
                                        close: t.close,
                                        volume: t.volume as i32,
                                        turnover: t.turnover,
                                    };
                                    let mut new_kline = combiner.combine_tick(&kline, true);
                                    if let Some(kline) = new_kline.take() {
                                        let _ = self.subscription.send(&Some(MarketData::Kline(kline)));
                                    }
                                }
                            }
                        }
                    }
                } else {
                    panic!("Error happened when subscribing data of symbol {}", symbol);
                }
                
            }
        };
        let handler = InteractiveThread::spawn(closure);
        handler
    }

    fn close(&mut self) {}
}

#[derive(Debug, serde::Deserialize, Clone)]
struct OrderBook {
    pub trading_day: String,
    pub datetime: String,
    pub open: String,
    pub high: String,
    pub low: String,
    pub close: String,
    pub volume: String,
    pub turnover: String,
    pub open_interest: String,
}

fn read_csv_file(path:String) -> Result<Vec<OrderBook>, Box<dyn Error>> {
    let mut data : Vec<OrderBook> = vec![];
    let mut rdr = csv::Reader::from_reader(File::open(path)?);
    for result in rdr.deserialize() {
        match result {
            Ok(value) => {
                let record: OrderBook = value;
                data.push(record);
            },
            Err(error) => {
                println!("Error happen!{:?}", error);
            }
        }
    }
    Ok(data)
}

fn _convert_str_f64(str: &str) -> Result<f64, AppError> {
    let ret = str.parse::<f64>();
    if ret.is_ok() {
        Ok(ret.unwrap())
    } else {
        Err(AppError::new(-200, format!("Error happened when converting tick str to f64: {}", str).as_str()).cause(Box::new(ret.unwrap_err())))
    }
}
fn _convert_str_i32(str: &str) -> Result<i32, AppError> {
    let ret = str.parse::<f64>();
    if ret.is_ok() {
        Ok(ret.unwrap() as i32)
    } else {
        Err(AppError::new(-200, format!("Error happened when converting tick str to i32: {}", str).as_str()).cause(Box::new(ret.unwrap_err())))
    }
}

fn _convert_tick(symbol : &str, order_book : &OrderBook) -> Result<Tick, AppError> {
    let tick = Tick {
        symbol: String::from(symbol),
        trading_day: order_book.trading_day.clone(),
        datetime: order_book.datetime.clone(),
        open: _convert_str_f64(order_book.open.as_str())?,
        high: _convert_str_f64(order_book.high.as_str())?,
        low: _convert_str_f64(order_book.low.as_str())?,
        close: _convert_str_f64(order_book.close.as_str())?,
        volume: _convert_str_f64(order_book.volume.as_str())?,
        turnover: _convert_str_f64(order_book.turnover.as_str())?,
        open_interest: _convert_str_f64(order_book.open_interest.as_str())?,
        last_price: _convert_str_f64(order_book.close.as_str())?,
        bid_price1: Default::default(),
        bid_price2: Default::default(),
        bid_price3: Default::default(),
        bid_price4: Default::default(),
        bid_price5: Default::default(),
        bid_volume1: Default::default(),
        bid_volume2: Default::default(),
        bid_volume3: Default::default(),
        bid_volume4: Default::default(),
        bid_volume5: Default::default(),
        ask_price1: Default::default(),
        ask_price2: Default::default(),
        ask_price3: Default::default(),
        ask_price4: Default::default(),
        ask_price5: Default::default(),
        ask_volume1: Default::default(),
        ask_volume2: Default::default(),
        ask_volume3: Default::default(),
        ask_volume4: Default::default(),
        ask_volume5: Default::default(),
    };
    Ok(tick)
}
