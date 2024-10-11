use super::trade_server::*;
use std::{collections::HashMap, thread::{self, JoinHandle}};
use common::{error::AppError, msmc::Subscription};
use std::sync::{Arc, Mutex, RwLock};
use crossbeam::channel::{self, Receiver, Sender};
type SharedGw = Arc<Mutex<TradeGateway>>;

static mut INSTANCE: Option<SharedGw> = None;
pub struct TradeGatewayHolder {
}

impl TradeGatewayHolder {
    pub fn init(server: Box<dyn TradeServer>) {
        unsafe {
            INSTANCE = Some(Arc::new(Mutex::new(TradeGateway {
                server: server,
                connected: false,
                subscription: None,
                subscribers: Arc::new(RwLock::new(Vec::new())),
                request_id_seq: 0,
                request_unit_map: Arc::new(Mutex::new(HashMap::new())),
            })));
        }
    }

    pub fn get_gateway() -> SharedGw {
        unsafe {
            INSTANCE.as_ref().unwrap().clone()
        }
    }
}

pub struct TradeGateway {
    server: Box<dyn TradeServer>,
    connected: bool,
    subscription: Option<Arc<RwLock<Subscription<(i32, TradeData)>>>>,
    subscribers : Arc<RwLock<Vec<(String, Sender<TradeData>)>>>,
    request_id_seq: i32,                                 //it needs to reset every trading day
    request_unit_map : Arc<Mutex<HashMap<i32, String>>>, //it needs to reset every trading day
}

impl TradeGateway {
    pub fn connect(&mut self, config : &TradeConfig)  -> Result<(), AppError> {
        if !self.connected {
            self.subscription = Some(Arc::new(RwLock::new(self.server.connect(config)?)));
            self.connected = true;
        }
        
        Ok(())
    }

    fn apply_request_id(&mut self, unit_id: &str) -> i32 {
        self.request_id_seq += 1;
        self.request_unit_map.lock().unwrap().insert(self.request_id_seq.clone(), unit_id.to_string());
        self.request_id_seq
    }
    
    pub fn subscribe(&mut self, unit_id: &str) -> Result<Receiver<TradeData>, AppError> {
        let (tx, rx) = channel::unbounded::<TradeData>();
        self.subscribers.write().unwrap().push((unit_id.to_string(), tx));
        Ok(rx)
    }

    pub fn start(&self) -> JoinHandle<()> {
        let subscription = self.subscription.as_ref().unwrap().clone();
        let subscribers = self.subscribers.clone();

        let request_unit_map_ref = self.request_unit_map.clone();
        thread::spawn(move || {
            let mut should_break = false;
            loop {
                let _ = subscription.read().unwrap().recv(&mut |event| {
                    match event {
                        Some((request_id, data)) => {
                            match data {
                                _ => {
                                    for sub in subscribers.read().unwrap().iter() {
                                        let request_unit_map = request_unit_map_ref.lock().unwrap();
                                        let opt = request_unit_map.get(&request_id);
                                        if let Some(unit_id) = opt {
                                            if *unit_id == sub.0 {
                                                let _ = sub.1.clone().send(data.clone());
                                            }
                                        }
                                    }
                                },
                            }
                        },
                        None => {
                            should_break = true;
                        },
                    }
                });

                if should_break {
                    break;
                }
            }
        })
    }

    pub fn send_order(&mut self, unit_id: &str, order : &OrderInsert) -> i32 {
        let request_id = self.apply_request_id(unit_id);
        self.server.send_order(&order, unit_id, request_id);
        request_id
    }

    pub fn cancel_order(&mut self, unit_id: &str, action: &OrderAction) -> i32 {
        let request_id = self.apply_request_id(unit_id);
        self.server.cancel_order(action, request_id);
        request_id
    }

    pub fn get_positions(&mut self, unit_id: &str, symbol: &str) -> Vec<Position> {
        self.server.get_positions(unit_id, symbol)
    }

    pub fn get_account(&mut self, unit_id: &str) -> Account {
        self.server.get_account(unit_id)
    }
}