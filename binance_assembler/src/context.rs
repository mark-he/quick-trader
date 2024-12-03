use std::sync::{Arc, Mutex};

use binance::{bn_market_server::BnMarketServer, bn_trade_server::BnTradeServer, model::{CancelOrderRequest, SymbolConfig, SymbolInfo}};
use binance_future_connector::trade::new_order::NewOrderRequest;
use binance_sim::{bn_sim_market_server::BnSimMarketServer, bn_sim_trade_server::BnSimTradeServer};

use common::{error::AppError, msmc::Subscription};
use crossbeam::channel::Receiver;
use market::{market_gateway::MarketGateway, market_server::{KLine, MarketData}};
use trade::{trade_gateway::TradeGateway, trade_server::{Position, TradeEvent, Wallet}};

pub static mut MARKET_GATEWAY: Option<Arc<Mutex<MarketGatewayDelegate>>> = None;
pub static mut TRADE_GATEWAY: Option<Arc<Mutex<TradeGatewayDelegate>>> = None;

pub fn get_market_gateway() -> Arc<Mutex<MarketGatewayDelegate>> {
    unsafe {
        MARKET_GATEWAY.as_ref().unwrap().clone()
    }
}

pub fn get_trade_gateway() -> Arc<Mutex<TradeGatewayDelegate>> {
    unsafe {
        TRADE_GATEWAY.as_ref().unwrap().clone()
    }
}

pub fn init_real(market_server: BnMarketServer, trade_server: BnTradeServer) {
    let market_gateway = MarketGateway::new(Box::new(market_server));
    let trade_gateway= TradeGateway::new(Box::new(trade_server));
    let market_delegate = Arc::new(Mutex::new(MarketGatewayDelegate {mode: Mode::Real, real: Some(market_gateway), sim: None}));
    let trade_delegate = Arc::new(Mutex::new(TradeGatewayDelegate {mode: Mode::Real, real: Some(trade_gateway), sim: None}));
    
    unsafe {
        if TRADE_GATEWAY.is_some() || MARKET_GATEWAY.is_some() {
            panic!("Duplicated initialization.")
        }
        MARKET_GATEWAY = Some(market_delegate);
        TRADE_GATEWAY = Some(trade_delegate);
    }
}

pub fn init_backtest(market_server: BnSimMarketServer, trade_server: BnSimTradeServer) {
    let market_gateway = MarketGateway::new(Box::new(market_server));
    let trade_gateway= TradeGateway::new(Box::new(trade_server));
    let market_delegate = Arc::new(Mutex::new(MarketGatewayDelegate {mode: Mode::Backtest, real: None, sim: Some(market_gateway)}));
    let trade_delegate = Arc::new(Mutex::new(TradeGatewayDelegate {mode: Mode::Backtest, real: None, sim: Some(trade_gateway)}));
    
    unsafe {
        if TRADE_GATEWAY.is_some() || MARKET_GATEWAY.is_some() {
            panic!("Duplicated initialization.")
        }
        MARKET_GATEWAY = Some(market_delegate);
        TRADE_GATEWAY = Some(trade_delegate);
    }
}

pub fn init_sim(market_server: BnMarketServer, trade_server: BnSimTradeServer) {
    let market_gateway = MarketGateway::new(Box::new(market_server));
    let trade_gateway= TradeGateway::new(Box::new(trade_server));
    let market_delegate = Arc::new(Mutex::new(MarketGatewayDelegate {mode: Mode::Sim, real: Some(market_gateway), sim: None}));
    let trade_delegate = Arc::new(Mutex::new(TradeGatewayDelegate {mode: Mode::Sim, real: None, sim: Some(trade_gateway)}));
    
    unsafe {
        if TRADE_GATEWAY.is_some() || MARKET_GATEWAY.is_some() {
            panic!("Duplicated initialization.")
        }
        MARKET_GATEWAY = Some(market_delegate);
        TRADE_GATEWAY = Some(trade_delegate);
    }
}

pub enum Mode {
    Sim,
    Real,
    Backtest,
}


pub struct TradeGatewayDelegate {
    pub mode: Mode,
    pub real: Option<TradeGateway<BnTradeServer>>,
    pub sim: Option<TradeGateway<BnSimTradeServer>>,
}

impl TradeGatewayDelegate {
    
    pub fn init(&mut self) -> Result<(), AppError> {
        match self.mode {
            Mode::Real => {
                self.real.as_mut().unwrap().init()
            },
            Mode::Backtest | Mode::Sim => {
                self.sim.as_mut().unwrap().init()
            },
        }
    }

    pub fn start(&mut self) -> Result<(), AppError> {
        match self.mode {
            Mode::Real => {
                self.real.as_mut().unwrap().start()
            },
            Mode::Backtest | Mode::Sim => {
                self.sim.as_mut().unwrap().start()
            },
        }
    }

    pub fn close(&self) {
        match self.mode {
            Mode::Real => {
                self.real.as_ref().unwrap().close()
            },
            Mode::Backtest | Mode::Sim => {
                self.sim.as_ref().unwrap().close()
            },
        }
    }

    pub fn register_symbol(&mut self, symbol: String) -> Receiver<TradeEvent> {
        match self.mode {
            Mode::Real => {
                self.real.as_mut().unwrap().register_symbol(symbol)
            },
            Mode::Backtest | Mode::Sim => {
                self.sim.as_mut().unwrap().register_symbol(symbol)
            },
        }
    }

    pub fn init_symbol(&mut self, symbol: String, config: SymbolConfig) -> Result<SymbolInfo, AppError> {
        match self.mode {
            Mode::Real => {
                self.real.as_mut().unwrap().init_symbol(symbol, config)
            },
            Mode::Backtest | Mode::Sim => {
                self.sim.as_mut().unwrap().init_symbol(symbol, config)
            },
        }
    }

    pub fn new_order(&mut self, symbol: String, request : NewOrderRequest) -> Result<(), AppError> {
        match self.mode {
            Mode::Real => {
                self.real.as_mut().unwrap().new_order(symbol, request)
            },
            Mode::Backtest | Mode::Sim => {
                self.sim.as_mut().unwrap().new_order(symbol, request)
            },
        }
    }

    pub fn cancel_order(&mut self,  symbol: String, request: CancelOrderRequest) -> Result<(), AppError> {
        match self.mode {
            Mode::Real => {
                self.real.as_mut().unwrap().cancel_order(symbol, request)
            },
            Mode::Backtest | Mode::Sim => {
                self.sim.as_mut().unwrap().cancel_order(symbol, request)
            },
        }
    }

    pub fn cancel_orders(&mut self, symbol: String) -> Result<(), AppError> {
        match self.mode {
            Mode::Real => {
                self.real.as_mut().unwrap().cancel_orders(symbol)
            },
            Mode::Backtest | Mode::Sim => {
                self.sim.as_mut().unwrap().cancel_orders(symbol)
            },
        }
    }

    pub fn get_positions(&mut self, symbol: String) -> Result<Vec<Position>, AppError> {
        match self.mode {
            Mode::Real => {
                self.real.as_mut().unwrap().get_positions(symbol)
            },
            Mode::Backtest | Mode::Sim => {
                self.sim.as_mut().unwrap().get_positions(symbol)
            },
        }
    }

    pub fn get_account(&mut self, account_id: &str) -> Result<Option<Wallet>, AppError> {
        match self.mode {
            Mode::Real => {
                self.real.as_mut().unwrap().get_account(account_id)
            },
            Mode::Backtest | Mode::Sim => {
                self.sim.as_mut().unwrap().get_account(account_id)
            },
        }
    }
}

pub struct MarketGatewayDelegate {
    pub mode: Mode,
    pub real: Option<MarketGateway<BnMarketServer>>,
    pub sim: Option<MarketGateway<BnSimMarketServer>>,
}

impl MarketGatewayDelegate {
    pub fn init(&mut self) -> Result<(), AppError> {
        match self.mode {
            Mode::Real | Mode::Sim=> {
                self.real.as_mut().unwrap().init()
            },
            Mode::Backtest => {
                self.sim.as_mut().unwrap().init()
            },
        }
    }
    pub fn get_tick_sub(&mut self) -> Subscription<MarketData> {
        match self.mode {
            Mode::Real | Mode::Sim=> {
                self.real.as_mut().unwrap().get_tick_sub()
            },
            Mode::Backtest => {
                self.sim.as_mut().unwrap().get_tick_sub()
            },
        }
    }
    pub fn load_kline(&mut self, symbol: String, interval: &str, count: u32) -> Result<Vec<KLine>, AppError> {
        match self.mode {
            Mode::Real | Mode::Sim=> {
                self.real.as_mut().unwrap().load_kline(symbol, interval, count)
            },
            Mode::Backtest => {
                self.sim.as_mut().unwrap().load_kline(symbol, interval, count)
            },
        }
    }
    pub fn subscribe_kline(&mut self, symbol: String, interval: &str) -> Receiver<MarketData> {
        match self.mode {
            Mode::Real | Mode::Sim=> {
                self.real.as_mut().unwrap().subscribe_kline(symbol, interval)
            },
            Mode::Backtest => {
                self.sim.as_mut().unwrap().subscribe_kline(symbol, interval)
            },
        }
    }
    pub fn subscribe_tick(&mut self, symbol: String) -> Receiver<MarketData> {
        match self.mode {
            Mode::Real | Mode::Sim=> {
                self.real.as_mut().unwrap().subscribe_tick(symbol)
            },
            Mode::Backtest => {
                self.sim.as_mut().unwrap().subscribe_tick(symbol)
            },
        }
    }
    pub fn start(&mut self) -> Result<(), AppError> {
        match self.mode {
            Mode::Real | Mode::Sim=> {
                self.real.as_mut().unwrap().start()
            },
            Mode::Backtest => {
                self.sim.as_mut().unwrap().start()
            },
        }
    }
    pub fn close(&self) {
        match self.mode {
            Mode::Real | Mode::Sim=> {
                self.real.as_ref().unwrap().close()
            },
            Mode::Backtest => {
                self.sim.as_ref().unwrap().close()
            },
        }
    }
    pub fn get_server_ping(&self) -> usize {
        match self.mode {
            Mode::Real | Mode::Sim=> {
                self.real.as_ref().unwrap().get_server_ping()
            },
            Mode::Backtest => {
                self.sim.as_ref().unwrap().get_server_ping()
            },
        }
    }
}

