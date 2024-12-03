use std::{sync::{atomic::{AtomicUsize, Ordering}, Arc, Mutex}, thread::JoinHandle};

use super::trade_server::*;
use common::{error::AppError, msmc::{StreamError, Subscription}};
use crossbeam::channel::{self, Receiver, Sender};

pub struct TradeGateway<S: TradeServer> {
    server: Box<S>,
    subscribers : Vec<(String, Sender<TradeEvent>)>,
    pub handler: Option<JoinHandle<()>>,
    start_ticket: Arc<AtomicUsize>,
    subscription: Arc<Mutex<Subscription<TradeEvent>>>,
}

impl<S: TradeServer> TradeGateway<S> {
    pub fn new(server: Box<S>) -> Self {
        TradeGateway {
            server,
            subscribers: vec![],
            handler: None,
            start_ticket: Arc::new(AtomicUsize::new(0)),
            subscription: Arc::new(Mutex::new(Subscription::top())),
        }
    }
    
    pub fn init(&mut self) -> Result<(), AppError> {
        self.server.init()
    }

    pub fn start(&mut self) -> Result<(), AppError> {
        let start_ticket = self.start_ticket.fetch_add(1, Ordering::SeqCst);
        let start_ticket_ref = self.start_ticket.clone();
        let subscription = self.server.start()?;
        self.subscription = Arc::new(Mutex::new(subscription));

        let subscribers = self.subscribers.clone();
        let handler = self.subscription.lock().unwrap().stream(move |event| {
            if start_ticket != start_ticket_ref.load(Ordering::SeqCst) - 1 {
                return Err(StreamError::Exit);
            }
            match event {
                Some(data) => {
                    let symbol = data.get_symbol();
                    for subscriber in subscribers.iter() {
                        if subscriber.0 == symbol || symbol == "" {
                            let _ = subscriber.1.send(data.clone());
                        }
                    }
                },
                None => {
                }
            }
            Ok(true)
        });
        self.handler = Some(handler);
        Ok(())
    }

    pub fn close(&self) {
        self.start_ticket.fetch_add(1, Ordering::SeqCst);
        self.server.close();
    }

    pub fn register_symbol(&mut self, symbol: S::Symbol) -> Receiver<TradeEvent> {
        let (tx, rx) = channel::unbounded::<TradeEvent>();
        self.subscribers.push((symbol.to_string(), tx.clone()));
        rx
    }

    pub fn init_symbol(&mut self, symbol: S::Symbol, config: S::SymbolConfig) -> Result<S::SymbolInfo, AppError> {
        self.server.init_symbol(symbol, config)
    }

    pub fn new_order(&mut self, symbol: S::Symbol, request : S::OrderRequest) -> Result<(), AppError> {
        self.server.new_order(symbol, request)
    }

    pub fn cancel_order(&mut self,  symbol: S::Symbol, request: S::CancelOrderRequest) -> Result<(), AppError> {
        self.server.cancel_order(symbol, request)
    }

    pub fn cancel_orders(&mut self, symbol: S::Symbol) -> Result<(), AppError> {
        self.server.cancel_orders(symbol)
    }

    pub fn get_positions(&mut self, symbol:S::Symbol) -> Result<Vec<Position>, AppError> {
        self.server.get_positions(symbol)
    }

    pub fn get_account(&mut self, account_id: &str) -> Result<Option<Wallet>, AppError> {
        self.server.get_account(account_id)
    }
}