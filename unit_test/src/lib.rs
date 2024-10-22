
#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::time::Duration;
    use binance::bn_market_server::BnMarketServer;
    use common::error::AppError;
    use backtest::backtest_market_server::BacktestMarketServer;
    use common::thread::InteractiveThread;
    use ctp::ctp_market_server::CtpMarketServer;
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

            server.subscribe_tick("m2502");

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
        let rx = gateway.subscribe_kline("m2501", "1m");

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
        
        let rx = gateway.subscribe_kline("m2502", "1m");
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

            server.subscribe_tick("m2501");
            
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
        let market_sub = gateway.get_tick_sub();
        let _ = gateway.subscribe_kline("m2501", "1m");

        let mut backtest_server = BacktestTradeServer::new();
        backtest_server.set_market_sub(market_sub);
        TradeGatewayHolder::init(Box::new(backtest_server));

        let trade_gateway = TradeGatewayHolder::get_gateway();
        let mut trade_gateway = trade_gateway.lock().unwrap();
        let _ = trade_gateway.connect(&TradeConfig {..Default::default()});

        let handle = gateway.start();


        let _ = handle.join();
    }


    #[test]
    fn test_market_bn_server() {
        let mut server = BnMarketServer::new();
        let prop : HashMap<String, String> = HashMap::new();
        let ret = server.connect(&prop); 
        println!("{:?}", ret.is_err());
        let _ = server.subscribe_tick("BTCUSDT");
        let _ = server.subscribe_kline("BTCUSDT", "1m");
        println!("{:?}", ret.is_err());
        let _ = server.start();


        if let Ok(sub) = ret {
            loop {
                let _ = sub.recv(&mut |event| {
                    println!("{:?}", event)
                });
            }
        }
        loop {
            println!("wait");
            thread::sleep(Duration::from_secs(1));
        }
    }

    #[test]
    fn test_interactive_thread() {
        let handler = InteractiveThread::spawn(move |rx|{
            loop {
                let ret = rx.try_recv();
                if let Ok(cmd) = ret {
                    println!("command {}", cmd);
                    break;
                }
                println!("Thread is running.")
            }
        });

        thread::sleep(Duration::from_secs(5));
        let _ = handler.sender.send("quit!!!!".to_string());
        let _ = handler.join_handler.join();
    }

}
