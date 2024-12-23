use std::{str::FromStr, sync::{Arc, Mutex}};

use binance::{bn_market_server::BnMarketServer, bn_trade_server::BnTradeServer, model::{BnMarketConfig, BnTradeConfig, SymbolConfig}};
use binance_future_connector::trade::new_order::NewOrderRequest;
use binance::{bn_sim_market_server::BnSimMarketServer, bn_sim_trade_server::BnSimTradeServer};

use bybit::{bb_market_server::BbMarketServer, bb_sim_market_server::BbSimMarketServer, bb_sim_trade_server::BbSimTradeServer, bb_trade_server::BbTradeServer, model::{BbMarketConfig, BbTradeConfig}};
use bybit::model::SymbolConfig as BbSymbolConfig;
use bybit_connector::trade::new_order::NewOrderRequest as BbNewOrderRequest;
use common::{error::AppError, msmc::Subscription};
use crossbeam::channel::Receiver;
use ctp::{ctp_market_server::CtpMarketServer, ctp_sim_market_server::CtpSimMarketServer, ctp_sim_trade_server::CtpSimTradeServer, ctp_trade_server::CtpTradeServer, model::{CancelOrderRequest, CtpConfig, NewOrderRequest as CtpNewOrderRequest, Symbol}};
use market::{market_gateway::MarketGateway, market_server::{KLine, MarketData}, sim_market_server::SimMarketConfig};
use serde_json::Value;
use trade::{sim_trade_server::SimTradeConfig, trade_gateway::TradeGateway, trade_server::{Position, TradeEvent, Wallet}};

use crate::model::{BacktestConfig, BbRealConfig, BbSimConfig, BnRealConfig, BnSimConfig, CtpSimConfig};

pub enum MarketGateways {
    BnSim(MarketGateway<BnMarketServer>),
    BnBacktest(MarketGateway<BnSimMarketServer>),
    BnReal(MarketGateway<BnMarketServer>),

    BbSim(MarketGateway<BbMarketServer>),
    BbBacktest(MarketGateway<BbSimMarketServer>),
    BbReal(MarketGateway<BbMarketServer>),

    CtpSim(MarketGateway<CtpMarketServer>),
    CtpBacktest(MarketGateway<CtpSimMarketServer>),
    CtpReal(MarketGateway<CtpMarketServer>),
}

pub enum TradeGateways {
    BnSim(TradeGateway<BnSimTradeServer>),
    BnBacktest(TradeGateway<BnSimTradeServer>),
    BnReal(TradeGateway<BnTradeServer>),

    BbSim(TradeGateway<BbSimTradeServer>),
    BbBacktest(TradeGateway<BbSimTradeServer>),
    BbReal(TradeGateway<BbTradeServer>),

    CtpSim(TradeGateway<CtpSimTradeServer>),
    CtpBacktest(TradeGateway<CtpSimTradeServer>),
    CtpReal(TradeGateway<CtpTradeServer>),
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
            MarketGateways::BbBacktest(s) => {
                return s.init();
            },
            MarketGateways::CtpReal(s) => {
                return s.init();
            },
            MarketGateways::CtpSim(s) => {
                return s.init();
            },
            MarketGateways::CtpBacktest(s) => {
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
            MarketGateways::BbBacktest(s) => {
                return s.get_tick_sub();
            },
            MarketGateways::CtpReal(s) => {
                return s.get_tick_sub();
            },
            MarketGateways::CtpSim(s) => {
                return s.get_tick_sub();
            },
            MarketGateways::CtpBacktest(s) => {
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
            MarketGateways::BbBacktest(s) => {
                return s.load_kline(symbol, interval, count);
            },
            MarketGateways::CtpReal(s) => {
                let symbol = Symbol::from_str(&symbol).map_err(|e| AppError::new(-200, &e.to_string()))?;
                return s.load_kline(symbol, interval, count);
            },
            MarketGateways::CtpSim(s) => {
                let symbol = Symbol::from_str(&symbol).map_err(|e| AppError::new(-200, &e.to_string()))?;
                return s.load_kline(symbol, interval, count);
            },
            MarketGateways::CtpBacktest(s) => {
                let symbol = Symbol::from_str(&symbol).map_err(|e| AppError::new(-200, &e.to_string()))?;
                return s.load_kline(symbol, interval, count);
            },
        }
    }
    pub fn subscribe_kline(&mut self, symbol: String, interval: &str) -> Result<Receiver<MarketData>, AppError> {
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
            MarketGateways::BbBacktest(s) => {
                return s.subscribe_kline(symbol, interval)
            },
            MarketGateways::CtpReal(s) => {
                let symbol = Symbol::from_str(&symbol).map_err(|e| AppError::new(-200, &e.to_string()))?;
                return s.subscribe_kline(symbol, interval)
            },
            MarketGateways::CtpSim(s) => {
                let symbol = Symbol::from_str(&symbol).map_err(|e| AppError::new(-200, &e.to_string()))?;
                return s.subscribe_kline(symbol, interval)
            },
            MarketGateways::CtpBacktest(s) => {
                let symbol = Symbol::from_str(&symbol).map_err(|e| AppError::new(-200, &e.to_string()))?;
                return s.subscribe_kline(symbol, interval)
            },
        }
    }
    pub fn subscribe_tick(&mut self, symbol: String) -> Result<Receiver<MarketData>, AppError> {
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
            MarketGateways::BbBacktest(s) => {
                return s.subscribe_tick(symbol)
            },
            MarketGateways::CtpReal(s) => {
                let symbol = Symbol::from_str(&symbol).map_err(|e| AppError::new(-200, &e.to_string()))?;
                return s.subscribe_tick(symbol)
            },
            MarketGateways::CtpSim(s) => {
                let symbol = Symbol::from_str(&symbol).map_err(|e| AppError::new(-200, &e.to_string()))?;
                return s.subscribe_tick(symbol)
            },
            MarketGateways::CtpBacktest(s) => {
                let symbol = Symbol::from_str(&symbol).map_err(|e| AppError::new(-200, &e.to_string()))?;
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
            MarketGateways::BbBacktest(s) => {
                return s.start()
            },
            MarketGateways::CtpReal(s) => {
                return s.start()
            },
            MarketGateways::CtpSim(s) => {
                return s.start()
            },
            MarketGateways::CtpBacktest(s) => {
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
            MarketGateways::BbBacktest(s) => {
                return s.close()
            },
            MarketGateways::CtpReal(s) => {
                return s.close()
            },
            MarketGateways::CtpSim(s) => {
                return s.close()
            },
            MarketGateways::CtpBacktest(s) => {
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
            MarketGateways::BbBacktest(s) => {
                return s.get_server_ping()
            },
            MarketGateways::CtpReal(s) => {
                return s.get_server_ping()
            },
            MarketGateways::CtpSim(s) => {
                return s.get_server_ping()
            },
            MarketGateways::CtpBacktest(s) => {
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
            TradeGateways::BbBacktest(s) => {
                return s.init()
            },
            TradeGateways::CtpReal(s) => {
                return s.init()
            },
            TradeGateways::CtpSim(s) => {
                return s.init()
            },
            TradeGateways::CtpBacktest(s) => {
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
            TradeGateways::BbBacktest(s) => {
                return s.start()
            },
            TradeGateways::CtpReal(s) => {
                return s.start()
            },
            TradeGateways::CtpSim(s) => {
                return s.start()
            },
            TradeGateways::CtpBacktest(s) => {
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
            TradeGateways::BbBacktest(s) => {
                return s.close()
            },
            TradeGateways::CtpReal(s) => {
                return s.close()
            },
            TradeGateways::CtpSim(s) => {
                return s.close()
            },
            TradeGateways::CtpBacktest(s) => {
                return s.close()
            },
        }
    }

    pub fn register_symbol(&mut self, symbol: String) -> Result<Receiver<TradeEvent>, AppError> {
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
            TradeGateways::BbBacktest(s) => {
                return s.register_symbol(symbol)
            },
            TradeGateways::CtpReal(s) => {
                let symbol = Symbol::from_str(&symbol).map_err(|e| AppError::new(-200, &e.to_string()))?;
                return s.register_symbol(symbol)
            },
            TradeGateways::CtpSim(s) => {
                let symbol = Symbol::from_str(&symbol).map_err(|e| AppError::new(-200, &e.to_string()))?;
                return s.register_symbol(symbol)
            },
            TradeGateways::CtpBacktest(s) => {
                let symbol = Symbol::from_str(&symbol).map_err(|e| AppError::new(-200, &e.to_string()))?;
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
            TradeGateways::BbBacktest(s) => {
                let ret = serde_json::from_str::<BbSymbolConfig>(config).map_err(|e| AppError::new(-200, &e.to_string()))?;
                serde_json::to_value(s.init_symbol(symbol, ret)?).map_err(|e| AppError::new(-200, &e.to_string()))
            },
            TradeGateways::CtpReal(s) => {
                let symbol = Symbol::from_str(&symbol).map_err(|e| AppError::new(-200, &e.to_string()))?;
                serde_json::to_value(s.init_symbol(symbol, ())?).map_err(|e| AppError::new(-200, &e.to_string()))
            },
            TradeGateways::CtpSim(s) => {
                let symbol = Symbol::from_str(&symbol).map_err(|e| AppError::new(-200, &e.to_string()))?;
                serde_json::to_value(s.init_symbol(symbol, ())?).map_err(|e| AppError::new(-200, &e.to_string()))
            },
            TradeGateways::CtpBacktest(s) => {
                let symbol = Symbol::from_str(&symbol).map_err(|e| AppError::new(-200, &e.to_string()))?;
                serde_json::to_value(s.init_symbol(symbol, ())?).map_err(|e| AppError::new(-200, &e.to_string()))
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
            TradeGateways::BbBacktest(s) => {
                let ret = serde_json::from_str::<BbNewOrderRequest>(request).map_err(|e| AppError::new(-200, &e.to_string()))?;
                return s.new_order(symbol, ret)
            },
            TradeGateways::CtpReal(s) => {
                let symbol = Symbol::from_str(&symbol).map_err(|e| AppError::new(-200, &e.to_string()))?;
                let ret = serde_json::from_str::<CtpNewOrderRequest>(request).map_err(|e| AppError::new(-200, &e.to_string()))?;
                return s.new_order(symbol, ret)
            },
            TradeGateways::CtpSim(s) => {
                let symbol = Symbol::from_str(&symbol).map_err(|e| AppError::new(-200, &e.to_string()))?;
                let ret = serde_json::from_str::<CtpNewOrderRequest>(request).map_err(|e| AppError::new(-200, &e.to_string()))?;
                return s.new_order(symbol, ret)
            },
            TradeGateways::CtpBacktest(s) => {
                let symbol = Symbol::from_str(&symbol).map_err(|e| AppError::new(-200, &e.to_string()))?;
                let ret = serde_json::from_str::<CtpNewOrderRequest>(request).map_err(|e| AppError::new(-200, &e.to_string()))?;
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
            TradeGateways::BbBacktest(s) => {
                return s.cancel_order(symbol, request.to_string())
            },
            TradeGateways::CtpReal(s) => {
                let symbol = Symbol::from_str(&symbol).map_err(|e| AppError::new(-200, &e.to_string()))?;
                let request = serde_json::from_str::<CancelOrderRequest>(request).map_err(|e| AppError::new(-200, &e.to_string()))?;
                return s.cancel_order(symbol, request)
            },
            TradeGateways::CtpSim(s) => {
                let symbol = Symbol::from_str(&symbol).map_err(|e| AppError::new(-200, &e.to_string()))?;
                let request = serde_json::from_str::<CancelOrderRequest>(request).map_err(|e| AppError::new(-200, &e.to_string()))?;
                return s.cancel_order(symbol, request)
            },
            TradeGateways::CtpBacktest(s) => {
                let symbol = Symbol::from_str(&symbol).map_err(|e| AppError::new(-200, &e.to_string()))?;
                let request = serde_json::from_str::<CancelOrderRequest>(request).map_err(|e| AppError::new(-200, &e.to_string()))?;
                return s.cancel_order(symbol, request)
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
            TradeGateways::BbBacktest(s) => {
                return s.cancel_orders(symbol)
            },
            TradeGateways::CtpReal(s) => {
                let symbol = Symbol::from_str(&symbol).map_err(|e| AppError::new(-200, &e.to_string()))?;
                return s.cancel_orders(symbol)
            },
            TradeGateways::CtpSim(s) => {
                let symbol = Symbol::from_str(&symbol).map_err(|e| AppError::new(-200, &e.to_string()))?;
                return s.cancel_orders(symbol)
            },
            TradeGateways::CtpBacktest(s) => {
                let symbol = Symbol::from_str(&symbol).map_err(|e| AppError::new(-200, &e.to_string()))?;
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
            TradeGateways::BbBacktest(s) => {
                return s.get_positions(symbol)
            },
            TradeGateways::CtpReal(s) => {
                let symbol = Symbol::from_str(&symbol).map_err(|e| AppError::new(-200, &e.to_string()))?;
                return s.get_positions(symbol)
            },
            TradeGateways::CtpSim(s) => {
                let symbol = Symbol::from_str(&symbol).map_err(|e| AppError::new(-200, &e.to_string()))?;
                return s.get_positions(symbol)
            },
            TradeGateways::CtpBacktest(s) => {
                let symbol = Symbol::from_str(&symbol).map_err(|e| AppError::new(-200, &e.to_string()))?;
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
            TradeGateways::BbBacktest(s) => {
                return s.get_account(account_id)
            },
            TradeGateways::CtpReal(s) => {
                return s.get_account(account_id)
            },
            TradeGateways::CtpSim(s) => {
                return s.get_account(account_id)
            },
            TradeGateways::CtpBacktest(s) => {
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
                    bybit::enable_prod(true);
                    let market_server = BbMarketServer::new(BbMarketConfig {
                        depth_level: config.depth_level.clone(),
                    });
                    let trade_server = BbTradeServer::new(BbTradeConfig {       
                        api_key: config.api_key.clone(), 
                        api_secret: config.api_secret.clone(),
                        position_side: config.position_side,
                        settle_coin: config.settle_coin.clone(),
                        margin_mode: config.margin_mode.clone(),
                    });
                    unsafe {
                        MARKET_GATEWAY = Some(Arc::new(Mutex::new(MarketGateways::BbReal(MarketGateway::new(Box::new(market_server))))));
                        TRADE_GATEWAY = Some(Arc::new(Mutex::new(TradeGateways::BbReal(TradeGateway::new(Box::new(trade_server))))));
                    }
                },
                "sim" => {
                    let config = serde_json::from_str::<BbSimConfig>(config).map_err(|e| AppError::new(-200, &e.to_string()))?;
                    log::init(log::Level::from_str(&config.log_level.to_uppercase()).unwrap(), config.log_utc);
                    bybit::enable_prod(true);
                    let market_server = BbMarketServer::new(BbMarketConfig {
                        depth_level: config.depth_level.clone(),
                    });
                    let trade_server = BbSimTradeServer::new(SimTradeConfig {       
                        order_completed_status: config.order_completed_status.clone(),
                        asset: config.asset.clone(),
                        balance: config.balance,
                    });  
                    unsafe {
                        MARKET_GATEWAY = Some(Arc::new(Mutex::new(MarketGateways::BbSim(MarketGateway::new(Box::new(market_server))))));
                        TRADE_GATEWAY = Some(Arc::new(Mutex::new(TradeGateways::BbSim(TradeGateway::new(Box::new(trade_server))))));
                    }
                },
                "backtest" => {
                    let config = serde_json::from_str::<BacktestConfig>(config).map_err(|e| AppError::new(-200, &e.to_string()))?;
                    log::init(log::Level::from_str(&config.log_level.to_uppercase()).unwrap(), config.log_utc);
                    bybit::enable_prod(true);
                    let market_server = BbSimMarketServer::new(SimMarketConfig {
                        start_time: config.start_time,
                        end_time: config.end_time,
                        interval: config.interval,
                        lines_per_sec: config.lines_per_sec,
                    });
                    let trade_server = BbSimTradeServer::new(SimTradeConfig {       
                        order_completed_status: config.order_completed_status.clone(),
                        asset: config.asset.clone(),
                        balance: config.balance,
                    });  
                    unsafe {
                        MARKET_GATEWAY = Some(Arc::new(Mutex::new(MarketGateways::BbBacktest(MarketGateway::new(Box::new(market_server))))));
                        TRADE_GATEWAY = Some(Arc::new(Mutex::new(TradeGateways::BbBacktest(TradeGateway::new(Box::new(trade_server))))));
                    }
                },
                _ => {
                    return Err(AppError::new(-200, "Not supported mode"));
                }
            }
        },
        "ctp" => {
            match mode {
                "real" => {
                    let config = serde_json::from_str::<CtpConfig>(config).map_err(|e| AppError::new(-200, &e.to_string()))?;
                    log::init(log::Level::from_str(&config.log_level.to_uppercase()).unwrap(), false);
                    let market_server = CtpMarketServer::new(config.clone());
                    let trade_server = CtpTradeServer::new(config.clone());
                    unsafe {
                        MARKET_GATEWAY = Some(Arc::new(Mutex::new(MarketGateways::CtpReal(MarketGateway::new(Box::new(market_server))))));
                        TRADE_GATEWAY = Some(Arc::new(Mutex::new(TradeGateways::CtpReal(TradeGateway::new(Box::new(trade_server))))));
                    }
                },
                "sim" => {
                    let config = serde_json::from_str::<CtpSimConfig>(config).map_err(|e| AppError::new(-200, &e.to_string()))?;
                    log::init(log::Level::from_str(&config.log_level.to_uppercase()).unwrap(), false);
                    let market_server = CtpMarketServer::new(CtpConfig {
                        log_level: config.log_level.clone(),
                        flow_path: config.flow_path.clone(),
                        front_addr: config.front_addr.clone(),
                        nm_addr: config.nm_addr.clone(),
                        user_info: config.user_info.clone(),
                        product_info: config.product_info.clone(),
                        auth_code: config.auth_code.clone(),
                        app_id: config.app_id.clone(),
                        broker_id: config.broker_id.clone(),
                        user_id: config.user_id.clone(),
                        password: config.password.clone(),
                    });
                    let trade_server = CtpSimTradeServer::new(SimTradeConfig {       
                        order_completed_status: config.order_completed_status.clone(),
                        asset: config.asset.clone(),
                        balance: config.balance,
                    });
                    unsafe {
                        MARKET_GATEWAY = Some(Arc::new(Mutex::new(MarketGateways::CtpSim(MarketGateway::new(Box::new(market_server))))));
                        TRADE_GATEWAY = Some(Arc::new(Mutex::new(TradeGateways::CtpSim(TradeGateway::new(Box::new(trade_server))))));
                    }
                },
                "backtest" => {
                    let config = serde_json::from_str::<BacktestConfig>(config).map_err(|e| AppError::new(-200, &e.to_string()))?;
                    log::init(log::Level::from_str(&config.log_level.to_uppercase()).unwrap(), false);
                    bybit::enable_prod(true);
                    let market_server = CtpSimMarketServer::new(SimMarketConfig {
                        start_time: config.start_time,
                        end_time: config.end_time,
                        interval: config.interval,
                        lines_per_sec: config.lines_per_sec,
                    });
                    let trade_server = CtpSimTradeServer::new(SimTradeConfig {       
                        order_completed_status: config.order_completed_status.clone(),
                        asset: config.asset.clone(),
                        balance: config.balance,
                    });  
                    unsafe {
                        MARKET_GATEWAY = Some(Arc::new(Mutex::new(MarketGateways::CtpBacktest(MarketGateway::new(Box::new(market_server))))));
                        TRADE_GATEWAY = Some(Arc::new(Mutex::new(TradeGateways::CtpBacktest(TradeGateway::new(Box::new(trade_server))))));
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
