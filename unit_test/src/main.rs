use std::{sync::{Arc, Mutex}, thread};

use binance::bn_market_server::BnMarketServer;
use market::market_gateway::MarketGateway;

type BnSharedMarketGw = Arc<Mutex<MarketGateway<BnMarketServer>>>;

static mut INSTANCE: Option<BnSharedMarketGw> = None;
pub struct BnMarketGatewayHolder {
}

impl BnMarketGatewayHolder {
    pub fn init(server: BnMarketServer) {
        unsafe {
            INSTANCE = Some(Arc::new(Mutex::new(MarketGateway::new(server))));
        }
    }

    pub fn get_gateway() -> BnSharedMarketGw {
        unsafe {
            INSTANCE.as_ref().unwrap().clone()
        }
    }
}

fn main() {
    binance::enable_prod(true);

    let server = BnMarketServer::new();
    BnMarketGatewayHolder::init(server);

    let market_gateway_ref = BnMarketGatewayHolder::get_gateway();
    let mut market_gateway = market_gateway_ref.lock().unwrap();

    let ret = market_gateway.connect();
    if ret.is_err() {
        panic!("XXXXXXXXXXXX{:?}XXXXXXXXXXXXXX", ret.unwrap_err());
    }
    let rx = market_gateway.subscribe_tick("BTCUSDT");
    let rx2 = market_gateway.subscribe_kline("BTCUSDT", "5m");
        
    let _ = market_gateway.start();

    thread::spawn(move || {
        loop {
            println!("Receiving tick");
            let ret = rx.recv();
            match ret {
                Ok(event) => {
                    println!("TICK={:?}", event);
                },
                Err(e) => {
                    println!("Error when receiving tick messages: {}", e.to_string());
                }
            }
        }
    });

    thread::spawn(move || {
        loop {
            println!("Receiving kline");
            let ret = rx2.recv();
            match ret {
                Ok(event) => {
                    println!("KLINE={:?}", event);
                },
                Err(e) => {
                    println!("Error when receiving kline messages: {:?}", e);
                }
            }
        }
    });
    let handler = &market_gateway.handler.as_ref().unwrap().join_handler;
    let _ = handler.lock().unwrap().take().unwrap().join().unwrap();
}