use std::sync::{Arc, Mutex};

use super::trade_server::*;
use common::{error::AppError, msmc::Subscription, thread::{Handler, InteractiveThread, Rx}};
use crossbeam::channel::{self, Receiver, Sender};
/*
type SharedGw = Arc<Mutex<TradeGateway<EventTrait, TradeServer>>>;

static mut INSTANCE: Option<SharedGw> = None;
pub struct TradeGatewayHolder<S: TradeServer> {
    _phantom: PhantomData<S>,
}

impl<S: TradeServer> TradeGatewayHolder<S> {
    pub fn init(server: S) {
        unsafe {
            INSTANCE = Some(Arc::new(Mutex::new(TradeGateway {
                server: server,
                connected: false,
                subscription: Subscription::top(),
                subscribers: Vec::new(),
            })));
        }
    }

    pub fn get_gateway() -> SharedGw {
        unsafe {
            INSTANCE.as_ref().unwrap().clone()
        }
    }
}
 */

pub struct TradeGateway<S: TradeServer> {
    server: S,
    connected: bool,
    subscription: Arc<Mutex<Subscription<S::Event>>>,
    subscribers : Vec<(String, Sender<S::Event>)>,
    pub handler: Option<Handler<()>>,
}

impl<S: TradeServer> TradeGateway<S> {
    pub fn new(server: S) -> Self {
        TradeGateway {
            server,
            connected: false,
            subscription: Arc::new(Mutex::new(Subscription::top())),
            subscribers: vec![],
            handler: None,
        }
    }

    pub fn start(&mut self) {
        let subscription_ref = self.subscription.clone();
        let subscribers = self.subscribers.clone();

        let closure = move |_| {
            let subscription = subscription_ref.lock().unwrap();
            let mut continue_flag = true;
            subscription.stream(&mut move |event| {
                match event {
                    Some(data) => {
                        println!("TRADE_GATEWAY: {:?}", data);
                        match data {
                            AccountEvent
                        }
                    },
                    None => {
                        continue_flag = false
                    }
                }
                continue_flag
            });
        };
        self.handler = Some(InteractiveThread::spawn(closure));
    }

    pub fn close(self) {
        if let Some(h) = self.handler {
            let _ = h.sender.send("QUIT".to_string());
        }
        self.server.close();
    }

    pub fn connect(&mut self)  -> Result<(), AppError> {
        if !self.connected {
            self.subscription = Arc::new(Mutex::new(self.server.connect()?));
            self.connected = true;
        }
        Ok(())
    }

    pub fn register(&mut self, symbols: Vec<String>) -> Result<Receiver<S::Event>, AppError> {
        let (tx, rx) = channel::unbounded::<S::Event>();
        for symbol in symbols {
            self.subscribers.push((symbol, tx.clone()));
        }
        Ok(rx)
    }

    pub fn new_order(&mut self, order : S::OrderRequest) -> Result<(), AppError> {
        self.server.new_order(order)
    }

    pub fn cancel_order(&mut self, symbol: &str, order_id: &str) -> Result<(), AppError> {
        self.server.cancel_order(symbol, order_id)
    }

    pub fn get_positions(&mut self, symbol: &str) -> Vec<S::Position> {
        self.server.get_positions(symbol)
    }

    pub fn get_account(&mut self, account_id: &str) -> Option<S::Account> {
        self.server.get_account(account_id)
    }
}