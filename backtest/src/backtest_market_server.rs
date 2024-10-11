use market::market_server::{MarketServer, MarketData, Tick};
use common::{error::AppError, msmc::*};
use std::collections::HashMap;
use std::fs::File;
use std::error::Error;
use std::thread::{self};
use std::time::Duration;
use std::sync::{Arc, RwLock};

static FILE_PATH :&str = "d:";

pub struct BacktestMarketServer {
    connected: bool,
    subscription:Arc<RwLock<Subscription<MarketData>>>,
    topics : HashMap<String, i32>,
}

impl BacktestMarketServer {
    pub fn new() -> Self {
        BacktestMarketServer {
            connected: false,
            subscription: Arc::new(RwLock::new(Subscription::top())),
            topics: HashMap::new(),
        }
    }
}

impl MarketServer for BacktestMarketServer {
    fn connect(&mut self, _prop : &HashMap<String, String>) -> Result<Subscription<MarketData>, AppError> {
        if !self.connected {
            let sub = self.subscription.write().unwrap().subscribe();
            self.connected = true;
            Ok(sub)
        } else {
            Err(AppError::new(-100, "Duplicated connected to MarketServer"))
        }
    }

    fn subscribe(&mut self, symbol: &str) -> Result<(), AppError> {
        if !self.connected {
            Err(AppError::new(-100, "MarketServer is disconnected"))
        } else {
            if !self.topics.contains_key(&symbol.to_string()) {
                let result = read_csv_file(String::from(format!("{}/{}.csv", FILE_PATH, symbol)));
                let subscription_ref = self.subscription.clone();
                let symbol_clone = symbol.to_string();
                if let Ok(data) = result {
                    thread::spawn(move || {
                        let subscription = subscription_ref.write().unwrap();
                        for o in data.iter() {
                            thread::sleep(Duration::from_millis(1000));
                            let v = o.clone();
                            let result = _convert_tick(symbol_clone.as_str(), v);
                            
                            if let Ok(tick) = result {
                                let _send_ret = subscription.send(&Some(MarketData::Tick(tick)));
                            }
                        }
                        let _send_ret = subscription.send(&Some(MarketData::MarketClosed));
                    });
                    Ok(())
                } else {
                    Err(AppError::new(-101, format!("Error happened when subscribing data of symbol {}", symbol).as_str()).cause(result.unwrap_err()))
                }
            } else {
                Ok(())
            }
        }
    }
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

fn _convert_tick(symbol : &str, order_book : OrderBook) -> Result<Tick, AppError> {
    let tick = Tick {
        symbol: String::from(symbol),
        trading_day: order_book.trading_day.clone(),
        datetime: order_book.datetime.clone(),
        open: _convert_str_f64(order_book.open.as_str())?,
        high: _convert_str_f64(order_book.high.as_str())?,
        low: _convert_str_f64(order_book.low.as_str())?,
        close: _convert_str_f64(order_book.close.as_str())?,
        volume: _convert_str_i32(order_book.volume.as_str())?,
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
