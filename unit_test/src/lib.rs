
#[cfg(test)]
mod tests {
    use std::{sync::{Arc, Mutex}, thread};

    use binance::{bn_market_server::BnMarketServer, bn_trade_server::{BnTradeServer, Config}};
    use common::error::AppError;
    use market::market_gateway::MarketGateway;
    use trade::trade_gateway::TradeGateway;

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
    #[test]
    fn test_bn_market_gateway() -> Result<(), AppError> {
        let server = BnMarketServer::new();
        BnMarketGatewayHolder::init(server);

        let market_gateway_ref = BnMarketGatewayHolder::get_gateway();
        let mut market_gateway = market_gateway_ref.lock().unwrap();

        let ret = market_gateway.connect();
        if ret.is_err() {
            panic!("{:?}", ret.unwrap_err());
        }
        let rx = market_gateway.subscribe_tick("BTCUSDT");
        let rx2 = market_gateway.subscribe_kline("BTCUSDT", "5m");
        
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
         
        let _ = market_gateway.start();
        let handler = &market_gateway.handler.as_ref().unwrap().join_handler;
        let _ = handler.lock().unwrap().take().unwrap().join();
        println!("=========QUIT============");
        Ok(())
    }
    

    type BnSharedTradeGw = Arc<Mutex<TradeGateway<BnTradeServer>>>;

    static mut INSTANCE2: Option<BnSharedTradeGw> = None;
    pub struct BnTradeGatewayHolder {
    }
    
    impl BnTradeGatewayHolder {
        pub fn init(server: BnTradeServer) {
            unsafe {
                INSTANCE2 = Some(Arc::new(Mutex::new(TradeGateway::new(server))));
            }
        }
    
        pub fn get_gateway() -> BnSharedTradeGw {
            unsafe {
                INSTANCE2.as_ref().unwrap().clone()
            }
        }
    }

    #[test]
    fn test_bn_trade_gateway() -> Result<(), AppError> {
        let server = BnTradeServer::new(Config {api_key: "13d233877484f4ea87afbbb8c29e52072c4e4a4a8650fcd689e076fab082bdc6".to_string(), api_secret: "671b347de4235aa3c2d3d15664db16180593ab21f65f4826e54b8f8e1ba11395".to_string(),});
        BnTradeGatewayHolder::init(server);

        let trade_gateway_ref = BnTradeGatewayHolder::get_gateway();
        let mut trade_gateway = trade_gateway_ref.lock().unwrap();
        let _ = trade_gateway.connect()?;

        let rx = trade_gateway.register(vec!["BTCUSDT".to_string()])?;

        thread::spawn(move || {
            loop {
                println!("Receiving account");
                let ret = rx.recv();
                match ret {
                    Ok(event) => {
                        println!("Account EVENT={:?}", event);
                    },
                    Err(e) => {
                        println!("Error when receiving account messages: {}", e.to_string());
                    }
                }
            }
        });

        let _ = trade_gateway.start();
        let handler = &trade_gateway.handler.as_ref().unwrap().join_handler;
        let _ = handler.lock().unwrap().take().unwrap().join();

        Ok(())
    }
}
