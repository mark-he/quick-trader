
#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use common::error::AppError;
    use backtest::backtest_market_server::BacktestMarketServer;
    use ctp::ctp_market_server::CtpMarketServer;
    use market::kline::KLineCombiner;
    use market::market_server::{MarketData, MarketServer};
    use market::market_gateway::MarketGatewayHolder;
    use backtest::backtest_trade_server::BacktestTradeServer;
    use ctp::ctp_trade_server::CtpTradeServer;
    use trade::trade_gateway::TradeGatewayHolder;
    use trade::trade_server::TradeConfig;
    use std::thread;

    #[test]
    fn test_error() {
        let error: AppError = AppError::new(-1, "this is an error");
        println!("{}", error);
        assert_eq!(1, 1);
    }

    #[test]
    fn test_backtest_market_server() {
        let mut server = BacktestMarketServer::new();
        let prop : HashMap<String, String> = HashMap::new();

        let ret = server.connect(&prop); 
        if let Ok(subscription) = ret {
            println!("server connected");
            let handle = thread::spawn(move || {
                let mut should_break = false;
                loop {
                    let _ = subscription.recv(&mut |event| {
                        match event {
                            Some(data) => {
                                match data {
                                    MarketData::Tick(t) => {
                                        println!("{:?}", t);
                                    },
                                    MarketData::MarketClosed => {
                                        should_break = true;
                                    },
                                    _ => {
                                        println!("Unknown {:?}", data);
                                    },
                                }
                            },
                            None => {
                            },
                        }
                    });
                    if should_break {
                        break;
                    }
                }
            });

            let result = server.subscribe("m2502");
            match result {
                Ok(_)=>{

                },
                Err(e) => {
                    panic!("{}", e)
                },
            }
            handle.join().unwrap();
            assert_eq!(1, 1);
        } else {
            assert_eq!(1, 2);
        }

    }
    
    #[test]
    fn test_market_gateway() {
        //let backtest_server = BacktestServer::new();
        let ctp_server = CtpMarketServer::new();
        MarketGatewayHolder::init(Box::new(ctp_server));
        let gateway = MarketGatewayHolder::get_gateway();
        let mut gateway = gateway.lock().unwrap();
        let _ = gateway.connect(&HashMap::new());
        let kline_combiner = Some(KLineCombiner::new("1m", 100, Some(21)));
        let ret = gateway.subscribe("m2501", kline_combiner);

        if let Ok(rx) = ret {
            thread::spawn(move || {
                loop {
                    let message =  rx.recv();
                    match message {
                        Ok(data) => {
                            match data {
                                MarketData::Tick(t) => {
                                    println!("1:{:?}", t);
                                },
                                MarketData::MarketClosed => {
                                    break;
                                },
                                _ => {},
                            }
                        },
                        Err(e) => {
                            panic!("{}", e)
                        },
                    }
                }
            });
        }
        
        let kline_combiner = Some(KLineCombiner::new("1m", 100, Some(21)));
        let ret = gateway.subscribe("m2502", kline_combiner);
        if let Ok(rx) = ret {
            thread::spawn(move || {
                loop {
                    let message =  rx.recv();
                    match message {
                        Ok(data) => {
                            match data {
                                MarketData::Tick(t) => {
                                    println!("2:{:?}", t);
                                },
                                MarketData::MarketClosed => {
                                    break;
                                },
                                _ => {},
                            }
                        },
                        Err(e) => {
                            panic!("{}", e)
                        },
                    }
                }
            });
        }
        gateway.start();
    }

    #[test]
    fn test_market_ctp_server() {
        let mut server = CtpMarketServer::new();
        let prop : HashMap<String, String> = HashMap::new();
        let ret = server.connect(&prop); 
        if let Ok(subscription) = ret {
            let handle = thread::spawn(move || {
                let mut should_break = false;
                loop {
                    let _ = subscription.recv(&mut |event| {
                        match event {
                            Some(data) => {
                                match data {
                                    MarketData::Tick(t) => {
                                        println!("{:?}", t);
                                    },
                                    MarketData::MarketClosed => {
                                        should_break = true;
                                    },
                                    _ => {
                                    },
                                }
                            },
                            None => {
                            },
                        }
                    });
                    if should_break {
                        break;
                    }
                }
            });

            let result = server.subscribe("m2501");
            match result {
                Ok(_)=>{

                },
                Err(e) => {
                    panic!("{}", e)
                },
            }
            handle.join().unwrap();
            assert_eq!(1, 1);
        } else {
            assert_eq!(1, 2);
        }
    }

    #[test]
    fn test_trade_gateway() {
        let ctp_server = CtpTradeServer::new();
        TradeGatewayHolder::init(Box::new(ctp_server));
        let gateway = TradeGatewayHolder::get_gateway();
        let mut gateway = gateway.lock().unwrap();
        let _ = gateway.connect(&TradeConfig {
            front_addr: "tcp://180.168.146.187:10202".into(),
            broker_id: "9999".into(),
            auth_code: "0000000000000000".into(),
            app_id: "simnow_client_test".into(),
    
            user_id: "222443".into(),
            password: "HIre0820##".into(),
            ..Default::default()
        });
    }

    #[test]
    fn test_backtest_trade_gateway() {
        let backtest_server = BacktestMarketServer::new();
        MarketGatewayHolder::init(Box::new(backtest_server));

        let gateway = MarketGatewayHolder::get_gateway();
        let mut gateway = gateway.lock().unwrap();
        let _ = gateway.connect(&HashMap::new());
        let market_sub = gateway.get_market_sub();
        let kline_combiner = Some(KLineCombiner::new("1m", 100, Some(21)));
        let _ = gateway.subscribe("m2501", kline_combiner);

        let mut backtest_server = BacktestTradeServer::new();
        backtest_server.set_market_sub(market_sub);
        TradeGatewayHolder::init(Box::new(backtest_server));

        let trade_gateway = TradeGatewayHolder::get_gateway();
        let mut trade_gateway = trade_gateway.lock().unwrap();
        let _ = trade_gateway.connect(&TradeConfig {..Default::default()});

        let handle = gateway.start();


        let _ = handle.join();
    }
}
