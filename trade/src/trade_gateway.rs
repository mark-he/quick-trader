use std::sync::{atomic::{AtomicUsize, Ordering}, Arc};

use super::trade_server::*;
use common::{error::AppError, thread::{Handler, InteractiveThread, Rx}};
use crossbeam::channel::{self, Receiver, Sender};

pub struct TradeGateway<S: TradeServer> {
    server: S,
    subscribers : Vec<(String, Sender<S::Event>)>,
    pub handler: Option<Handler<()>>,
    start_ticket: Arc<AtomicUsize>,
}

impl<S: TradeServer> TradeGateway<S> {
    pub fn new(server: S) -> Self {
        TradeGateway {
            server,
            subscribers: vec![],
            handler: None,
            start_ticket: Arc::new(AtomicUsize::new(0)),
        }
    }
    
    pub fn init(&mut self) -> Result<(), AppError> {
        self.server.init()
    }

    pub fn start(&mut self) -> Result<(), AppError> {
        let start_ticket = self.start_ticket.fetch_add(1, Ordering::SeqCst);
        let start_ticket_ref = self.start_ticket.clone();
        let subscription = self.server.start()?;
        let subscribers = self.subscribers.clone();

        let closure = move |rx: Rx<String>| { 
            let _ = subscription.stream(&mut move |event| {
                if start_ticket != start_ticket_ref.load(Ordering::SeqCst) - 1 {
                    return Ok(true);
                }
                let cmd = rx.try_recv();
                if cmd.is_ok() {
                    if cmd.unwrap() == "QUIT" {
                        return Ok(false);
                    }
                }

                match event {
                    Some(data) => {
                        let symbol = data.get_symbol();
                        for subscriber in subscribers.iter() {
                            if subscriber.0 == symbol {
                                let _ = subscriber.1.send(data.clone());
                            }
                        }
                    },
                    None => {
                        
                    }
                }
                Ok(true)
            }, true);
        };
        self.handler = Some(InteractiveThread::spawn(closure));
        Ok(())
    }

    pub fn close(self) {
        if let Some(h) = self.handler {
            let _ = h.sender.send("QUIT".to_string());
        }
        self.server.close();
    }

    pub fn register_symbol(&mut self, symbol: &str) -> Receiver<S::Event> {
        let (tx, rx) = channel::unbounded::<S::Event>();
        self.subscribers.push((symbol.to_string(), tx.clone()));
        rx
    }

    pub fn init_symbol(&mut self, symbol: &str, config: S::SymbolConfig) -> Result<S::SymbolInfo, AppError> {
        self.server.init_symbol(symbol, config)
    }

    pub fn new_order(&mut self, order : S::OrderRequest) -> Result<(), AppError> {
        self.server.new_order(order)
    }

    pub fn cancel_order(&mut self, symbol: &str, order_id: &str) -> Result<(), AppError> {
        self.server.cancel_order(symbol, order_id)
    }

    pub fn cancel_orders(&mut self, symbol: &str) -> Result<(), AppError> {
        self.server.cancel_orders(symbol)
    }

    pub fn get_positions(&mut self, symbol: &str) -> Vec<S::Position> {
        self.server.get_positions(symbol)
    }

    pub fn get_account(&mut self, account_id: &str) -> Option<S::Account> {
        self.server.get_account(account_id)
    }
}