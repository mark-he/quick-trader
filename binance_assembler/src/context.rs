use std::{str::FromStr, sync::{Arc, Mutex}};

use binance::{bn_market_server::BnMarketServer, bn_trade_server::BnTradeServer, model::{BnMarketConfig, BnTradeConfig, SymbolConfig}};
use binance_future_connector::trade::new_order::NewOrderRequest;
use binance_sim::{bn_sim_market_server::BnSimMarketServer, bn_sim_trade_server::BnSimTradeServer, model::{SimMarketConfig, SimTradeConfig}};

use bybit::{bb_market_server::BbMarketServer, bb_trade_server::BbTradeServer, model::{BbMarketConfig, BbTradeConfig}};
use bybit::model::SymbolConfig as BbSymbolConfig;
use bybit_connector::trade::new_order::NewOrderRequest as BbNewOrderRequest;
use common::{error::AppError, msmc::Subscription};
use crossbeam::channel::Receiver;
use market::{market_gateway::MarketGateway, market_server::{KLine, MarketData}};
use serde_json::Value;
use trade::{trade_gateway::TradeGateway, trade_server::{Position, TradeEvent, Wallet}};

use crate::c_model::{BacktestConfig, BbRealConfig, BnRealConfig, BnSimConfig};

pub enum MarketGateways {
    BnSim(MarketGateway<BnMarketServer>),
    BnBacktest(MarketGateway<BnSimMarketServer>),
    BnReal(MarketGateway<BnMarketServer>),

    BbReal(MarketGateway<BbMarketServer>),
    BbSim(MarketGateway<BbMarketServer>),
}

pub enum TradeGateways {
    BnSim(TradeGateway<BnSimTradeServer>),
    BnBacktest(TradeGateway<BnSimTradeServer>),
    BnReal(TradeGateway<BnTradeServer>),

    BbReal(TradeGateway<BbTradeServer>),
    BbSim(TradeGateway<BbTradeServer>),
}

impl MarketGateways {
    pub fn init(&mut self) -> Result<(), AppError> {
        match self {
            MarketGateways::BnSim(s) => {
                return s.init();
            },
            MarketGateways::BnBacktest(s) => {
                return s.init();
            },
            MarketGateways::BnReal(s) => {
                return s.init();
            },
            MarketGateways::BbReal(s) => {
                return s.init();
            },
            MarketGateways::BbSim(s) => {
                return s.init();
            },
        }
    }
    pub fn get_tick_sub(&mut self) -> Subscription<MarketData> {
        match self {
            MarketGateways::BnSim(s) => {
                return s.get_tick_sub();
            },
            MarketGateways::BnBacktest(s) => {
                return s.get_tick_sub();
            },
            MarketGateways::BnReal(s) => {
                return s.get_tick_sub();
            },
            MarketGateways::BbReal(s) => {
                return s.get_tick_sub();
            },
            MarketGateways::BbSim(s) => {
                return s.get_tick_sub();
            },
        }
    }
    pub fn load_kline(&mut self, symbol: String, interval: &str, count: u32) -> Result<Vec<KLine>, AppError> {
        match self {
            MarketGateways::BnSim(s) => {
                return s.load_kline(symbol, interval, count);
            },
            MarketGateways::BnBacktest(s) => {
                return s.load_kline(symbol, interval, count);
            },
            MarketGateways::BnReal(s) => {
                return s.load_kline(symbol, interval, count);
            },
            MarketGateways::BbReal(s) => {
                return s.load_kline(symbol, interval, count);
            },
            MarketGateways::BbSim(s) => {
                return s.load_kline(symbol, interval, count);
            },
        }
    }
    pub fn subscribe_kline(&mut self, symbol: String, interval: &str) -> Receiver<MarketData> {
        match self {
            MarketGateways::BnSim(s) => {
                return s.subscribe_kline(symbol, interval)
            },
            MarketGateways::BnBacktest(s) => {
                return s.subscribe_kline(symbol, interval)
            },
            MarketGateways::BnReal(s) => {
                return s.subscribe_kline(symbol, interval)
            },
            MarketGateways::BbReal(s) => {
                return s.subscribe_kline(symbol, interval)
            },
            MarketGateways::BbSim(s) => {
                return s.subscribe_kline(symbol, interval)
            },
        }
    }
    pub fn subscribe_tick(&mut self, symbol: String) -> Receiver<MarketData> {
        match self {
            MarketGateways::BnSim(s) => {
                return s.subscribe_tick(symbol)
            },
            MarketGateways::BnBacktest(s) => {
                return s.subscribe_tick(symbol)
            },
            MarketGateways::BnReal(s) => {
                return s.subscribe_tick(symbol)
            },
            MarketGateways::BbReal(s) => {
                return s.subscribe_tick(symbol)
            },
            MarketGateways::BbSim(s) => {
                return s.subscribe_tick(symbol)
            },
        }
    }
    pub fn start(&mut self) -> Result<(), AppError> {
        match self {
            MarketGateways::BnSim(s) => {
                return s.start()
            },
            MarketGateways::BnBacktest(s) => {
                return s.start()
            },
            MarketGateways::BnReal(s) => {
                return s.start()
            },
            MarketGateways::BbReal(s) => {
                return s.start()
            },
            MarketGateways::BbSim(s) => {
                return s.start()
            },
        }
    }
    pub fn close(&self) {
        match self {
            MarketGateways::BnSim(s) => {
                return s.close()
            },
            MarketGateways::BnBacktest(s) => {
                return s.close()
            },
            MarketGateways::BnReal(s) => {
                return s.close()
            },
            MarketGateways::BbReal(s) => {
                return s.close()
            },
            MarketGateways::BbSim(s) => {
                return s.close()
            },
        }
    }
    pub fn get_server_ping(&self) -> usize {
        match self {
            MarketGateways::BnSim(s) => {
                return s.get_server_ping()
            },
            MarketGateways::BnBacktest(s) => {
                return s.get_server_ping()
            },
            MarketGateways::BnReal(s) => {
                return s.get_server_ping()
            },
            MarketGateways::BbReal(s) => {
                return s.get_server_ping()
            },
            MarketGateways::BbSim(s) => {
                return s.get_server_ping()
            },
        }
    }
}

impl TradeGateways {
    pub fn init(&mut self) -> Result<(), AppError> {
        match self {
            TradeGateways::BnSim(s) => {
                return s.init()
            },
            TradeGateways::BnBacktest(s) => {
                return s.init()
            },
            TradeGateways::BnReal(s) => {
                return s.init()
            },
            TradeGateways::BbReal(s) => {
                return s.init()
            },
            TradeGateways::BbSim(s) => {
                return s.init()
            },
        }
    }

    pub fn start(&mut self) -> Result<(), AppError> {
        match self {
            TradeGateways::BnSim(s) => {
                return s.start()
            },
            TradeGateways::BnBacktest(s) => {
                return s.start()
            },
            TradeGateways::BnReal(s) => {
                return s.start()
            },
            TradeGateways::BbReal(s) => {
                return s.start()
            },
            TradeGateways::BbSim(s) => {
                return s.start()
            },
        }
    }

    pub fn close(&self) {
        match self {
            TradeGateways::BnSim(s) => {
                return s.close()
            },
            TradeGateways::BnBacktest(s) => {
                return s.close()
            },
            TradeGateways::BnReal(s) => {
                return s.close()
            },
            TradeGateways::BbReal(s) => {
                return s.close()
            },
            TradeGateways::BbSim(s) => {
                return s.close()
            },
        }
    }

    pub fn register_symbol(&mut self, symbol: String) -> Receiver<TradeEvent> {
        match self {
            TradeGateways::BnSim(s) => {
                return s.register_symbol(symbol)
            },
            TradeGateways::BnBacktest(s) => {
                return s.register_symbol(symbol)
            },
            TradeGateways::BnReal(s) => {
                return s.register_symbol(symbol)
            },
            TradeGateways::BbReal(s) => {
                return s.register_symbol(symbol)
            },
            TradeGateways::BbSim(s) => {
                return s.register_symbol(symbol)
            },
        }
    }

    pub fn init_symbol(&mut self, symbol: String, config: &str) -> Result<Value, AppError> {
        match self {
            TradeGateways::BnSim(s) => {
                let ret = serde_json::from_str::<SymbolConfig>(config).map_err(|e| AppError::new(-200, &e.to_string()))?;
                serde_json::to_value(s.init_symbol(symbol, ret)?).map_err(|e| AppError::new(-200, &e.to_string()))
            },
            TradeGateways::BnBacktest(s) => {
                let ret = serde_json::from_str::<SymbolConfig>(config).map_err(|e| AppError::new(-200, &e.to_string()))?;
                serde_json::to_value(s.init_symbol(symbol, ret)?).map_err(|e| AppError::new(-200, &e.to_string()))
            },
            TradeGateways::BnReal(s) => {
                let ret = serde_json::from_str::<SymbolConfig>(config).map_err(|e| AppError::new(-200, &e.to_string()))?;
                serde_json::to_value(s.init_symbol(symbol, ret)?).map_err(|e| AppError::new(-200, &e.to_string()))
            },
            TradeGateways::BbReal(s) => {
                let ret = serde_json::from_str::<BbSymbolConfig>(config).map_err(|e| AppError::new(-200, &e.to_string()))?;
                serde_json::to_value(s.init_symbol(symbol, ret)?).map_err(|e| AppError::new(-200, &e.to_string()))
            },
            TradeGateways::BbSim(s) => {
                let ret = serde_json::from_str::<BbSymbolConfig>(config).map_err(|e| AppError::new(-200, &e.to_string()))?;
                serde_json::to_value(s.init_symbol(symbol, ret)?).map_err(|e| AppError::new(-200, &e.to_string()))
            },
        }
    }

    pub fn new_order(&mut self, symbol: String, request : &str) -> Result<(), AppError> {
        match self {
            TradeGateways::BnSim(s) => {
                let ret = serde_json::from_str::<NewOrderRequest>(request).map_err(|e| AppError::new(-200, &e.to_string()))?;
                return s.new_order(symbol, ret)
            },
            TradeGateways::BnBacktest(s) => {
                let ret = serde_json::from_str::<NewOrderRequest>(request).map_err(|e| AppError::new(-200, &e.to_string()))?;
                return s.new_order(symbol, ret)
            },
            TradeGateways::BnReal(s) => {
                let ret = serde_json::from_str::<NewOrderRequest>(request).map_err(|e| AppError::new(-200, &e.to_string()))?;
                return s.new_order(symbol, ret)
            },
            TradeGateways::BbReal(s) => {
                let ret = serde_json::from_str::<BbNewOrderRequest>(request).map_err(|e| AppError::new(-200, &e.to_string()))?;
                return s.new_order(symbol, ret)
            },
            TradeGateways::BbSim(s) => {
                let ret = serde_json::from_str::<BbNewOrderRequest>(request).map_err(|e| AppError::new(-200, &e.to_string()))?;
                return s.new_order(symbol, ret)
            },
        }
    }

    pub fn cancel_order(&mut self,  symbol: String, request: &str) -> Result<(), AppError> {
        match self {
            TradeGateways::BnSim(s) => {
                return s.cancel_order(symbol, request.to_string())
            },
            TradeGateways::BnBacktest(s) => {
                return s.cancel_order(symbol, request.to_string())
            },
            TradeGateways::BnReal(s) => {
                return s.cancel_order(symbol, request.to_string())
            },
            TradeGateways::BbReal(s) => {
                return s.cancel_order(symbol, request.to_string())
            },
            TradeGateways::BbSim(s) => {
                return s.cancel_order(symbol, request.to_string())
            },
        }
    }

    pub fn cancel_orders(&mut self, symbol: String) -> Result<(), AppError> {
        match self {
            TradeGateways::BnSim(s) => {
                return s.cancel_orders(symbol)
            },
            TradeGateways::BnBacktest(s) => {
                return s.cancel_orders(symbol)
            },
            TradeGateways::BnReal(s) => {
                return s.cancel_orders(symbol)
            },
            TradeGateways::BbReal(s) => {
                return s.cancel_orders(symbol)
            },
            TradeGateways::BbSim(s) => {
                return s.cancel_orders(symbol)
            },
        }
    }

    pub fn get_positions(&mut self, symbol: String) -> Result<Vec<Position>, AppError> {
        match self {
            TradeGateways::BnSim(s) => {
                return s.get_positions(symbol)
            },
            TradeGateways::BnBacktest(s) => {
                return s.get_positions(symbol)
            },
            TradeGateways::BnReal(s) => {
                return s.get_positions(symbol)
            },
            TradeGateways::BbReal(s) => {
                return s.get_positions(symbol)
            },
            TradeGateways::BbSim(s) => {
                return s.get_positions(symbol)
            },
        }
    }

    pub fn get_account(&mut self, account_id: &str) -> Result<Option<Wallet>, AppError> {
        match self {
            TradeGateways::BnSim(s) => {
                return s.get_account(account_id)
            },
            TradeGateways::BnBacktest(s) => {
                return s.get_account(account_id)
            },
            TradeGateways::BnReal(s) => {
                return s.get_account(account_id)
            },
            TradeGateways::BbReal(s) => {
                return s.get_account(account_id)
            },
            TradeGateways::BbSim(s) => {
                return s.get_account(account_id)
            },
        }
    }
}


pub static mut MARKET_GATEWAY: Option<Arc<Mutex<MarketGateways>>> = None;
pub static mut TRADE_GATEWAY: Option<Arc<Mutex<TradeGateways>>> = None;

pub fn get_market_gateway() -> Arc<Mutex<MarketGateways>> {
    unsafe {
        MARKET_GATEWAY.as_ref().unwrap().clone()
    }
}

pub fn get_trade_gateway() -> Arc<Mutex<TradeGateways>> {
    unsafe {
        TRADE_GATEWAY.as_ref().unwrap().clone()
    }
}

pub fn init(exchange: &str, mode: &str, config: &str) -> Result<(), AppError>{
    match exchange {
        "binance" => {
            match mode {
                "real" => {
                    let config = serde_json::from_str::<BnRealConfig>(config).map_err(|e| AppError::new(-200, &e.to_string()))?;
                    log::init(log::Level::from_str(&config.log_level.to_uppercase()).unwrap(), config.log_utc);
                    binance::enable_prod(true);
                    let market_server = BnMarketServer::new(BnMarketConfig {
                        tick_update_speed: config.tick_update_speed.clone(),
                        depth_level: config.depth_level.clone(),
                    });
                    let trade_server = BnTradeServer::new(BnTradeConfig {       
                        api_key: config.api_key.clone(), 
                        api_secret: config.api_secret.clone(),
                        dual_position_side: config.dual_position_side.clone(),
                        multi_assets_margin: config.multi_assets_margin.clone(),
                    });
                    unsafe {
                        MARKET_GATEWAY = Some(Arc::new(Mutex::new(MarketGateways::BnReal(MarketGateway::new(Box::new(market_server))))));
                        TRADE_GATEWAY = Some(Arc::new(Mutex::new(TradeGateways::BnReal(TradeGateway::new(Box::new(trade_server))))));
                    }
                },
                "sim" => {
                    let config = serde_json::from_str::<BnSimConfig>(config).map_err(|e| AppError::new(-200, &e.to_string()))?;
                    log::init(log::Level::from_str(&config.log_level.to_uppercase()).unwrap(), config.log_utc);
                    binance::enable_prod(true);
                    let market_server = BnMarketServer::new(BnMarketConfig {
                        tick_update_speed: config.tick_update_speed.clone(),
                        depth_level: config.depth_level.clone(),
                    });
                    let trade_server = BnSimTradeServer::new(SimTradeConfig {
                        asset: config.asset,
                        balance: config.balance,
                        order_completed_status: config.order_completed_status.clone(),
                    });
                    unsafe {
                        MARKET_GATEWAY = Some(Arc::new(Mutex::new(MarketGateways::BnSim(MarketGateway::new(Box::new(market_server))))));
                        TRADE_GATEWAY = Some(Arc::new(Mutex::new(TradeGateways::BnSim(TradeGateway::new(Box::new(trade_server))))));
                    }
                },
                "backtest" => {
                    let config = serde_json::from_str::<BacktestConfig>(config).map_err(|e| AppError::new(-200, &e.to_string()))?;
                    log::init(log::Level::from_str(&config.log_level.to_uppercase()).unwrap(), config.log_utc);
                    binance::enable_prod(true);

                    let market_server = BnSimMarketServer::new(SimMarketConfig {
                        start_time: config.start_time,
                        end_time: config.end_time,
                        interval: config.interval,
                        lines_per_sec: config.lines_per_sec,
                    });
                    let trade_server = BnSimTradeServer::new(SimTradeConfig {
                        asset: config.asset,
                        balance: config.balance,
                        order_completed_status: config.order_completed_status.clone(),
                    });
                    unsafe {
                        MARKET_GATEWAY = Some(Arc::new(Mutex::new(MarketGateways::BnBacktest(MarketGateway::new(Box::new(market_server))))));
                        TRADE_GATEWAY = Some(Arc::new(Mutex::new(TradeGateways::BnBacktest(TradeGateway::new(Box::new(trade_server))))));
                    }
                },
                _ => {
                    return Err(AppError::new(-200, "Not supported mode"));
                }
            }
        },
        "bybit" => {
            match mode {
                "real" => {
                    let config = serde_json::from_str::<BbRealConfig>(config).map_err(|e| AppError::new(-200, &e.to_string()))?;
                    log::init(log::Level::from_str(&config.log_level.to_uppercase()).unwrap(), config.log_utc);
                    binance::enable_prod(true);
                    let market_server = BbMarketServer::new(BbMarketConfig {
                        depth_level: config.depth_level.clone(),
                    });
                    let trade_server = BbTradeServer::new(BbTradeConfig {       
                        api_key: config.api_key.clone(), 
                        api_secret: config.api_secret.clone(),
                        position_side: config.position_side,
                    });
                    unsafe {
                        MARKET_GATEWAY = Some(Arc::new(Mutex::new(MarketGateways::BbReal(MarketGateway::new(Box::new(market_server))))));
                        TRADE_GATEWAY = Some(Arc::new(Mutex::new(TradeGateways::BbReal(TradeGateway::new(Box::new(trade_server))))));
                    }
                },
                "sim" => {
                    let config = serde_json::from_str::<BbRealConfig>(config).map_err(|e| AppError::new(-200, &e.to_string()))?;
                    log::init(log::Level::from_str(&config.log_level.to_uppercase()).unwrap(), config.log_utc);
                    binance::enable_prod(false);
                    let market_server = BbMarketServer::new(BbMarketConfig {
                        depth_level: config.depth_level.clone(),
                    });
                    let trade_server = BbTradeServer::new(BbTradeConfig {       
                        api_key: config.api_key.clone(), 
                        api_secret: config.api_secret.clone(),
                        position_side: config.position_side,
                    });
                    unsafe {
                        MARKET_GATEWAY = Some(Arc::new(Mutex::new(MarketGateways::BbReal(MarketGateway::new(Box::new(market_server))))));
                        TRADE_GATEWAY = Some(Arc::new(Mutex::new(TradeGateways::BbReal(TradeGateway::new(Box::new(trade_server))))));
                    }
                },
                _ => {
                    return Err(AppError::new(-200, "Not supported mode"));
                }
            }
        },
        _ => {
            return Err(AppError::new(-200, "Not supported exchange"));
        }
    }
    Ok(())
}
