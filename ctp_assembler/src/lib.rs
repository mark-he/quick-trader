pub mod spi;
pub mod context;
pub mod c_model;

#[cfg(test)]
mod tests {
    use common::error::AppError;
    use ctp::{ctp_market_server::CtpMarketServer, ctp_trade_server::CtpTradeServer, model::{Config, Symbol, TradeEvent}};
    use market::market_server::MarketData;
    use std::{str::FromStr, thread};
    use crate::context;
    use log::*;

    fn init_gateweay() {
        log::init(Level::Trace, false);
        let config = Config {
            log_level: "INFO".to_string(),
            flow_path: "".to_string(),
            front_addr: "180.168.146.187:10202".to_string(),
            nm_addr: "180.168.146.187:10212".to_string(),
            user_info: "vnpy".to_string(),
            product_info: "vnpy".to_string(),
            auth_code: "0000000000000000".to_string(),
            app_id: "simnow_client_test".to_string(),
            broker_id: "9999".to_string(),
            user_id: "222443".to_string(),
            password: "HIre0820##".to_string(),
        };
        let market_server = CtpMarketServer::new(config.clone());
        let trade_server = CtpTradeServer::new(config.clone());
        context::init(market_server, trade_server);
    }

    #[test]
    fn test_market_kline() -> Result<(), AppError> {
        init_gateweay();
        let gateway_ref = context::get_market_gateway();
        let mut gateway = gateway_ref.lock().unwrap();
    
        let rx  = gateway.subscribe_kline(Symbol::from_str("m2501@DCE").map_err(|e| AppError::new(-200, &e))?, "1m");
        let ret = gateway.start();
        if ret.is_err() {
            error!("{:?}", ret.unwrap_err());
        }
        let handler = thread::spawn(move || {
            loop {
                if let Ok(data) = rx.recv() {
                    match data {
                        MarketData::Kline(k) => {
                            let json = serde_json::to_string(&k).unwrap();
                            info!("kline {}", json);
                        },
                        MarketData::Tick(tick) => {
                            let json = serde_json::to_string(&tick).unwrap();
                            info!("test {}", json);
                        },
                        _ => {},
                    }
                }
            }
        });
        handler.join().unwrap();
        Ok(())
    }

    #[test]
    fn test_market_tick() -> Result<(), AppError> {
        init_gateweay();

        let gateway_ref = context::get_market_gateway();
        let mut gateway = gateway_ref.lock().unwrap();
    
        let rx  = gateway.subscribe_tick(Symbol::from_str("m2501@DCE").map_err(|e| AppError::new(-200, &e))?);
        let ret = gateway.start();
        if ret.is_err() {
            error!("{:?}", ret.unwrap_err());
        }
        let handler = thread::spawn(move || {
            loop {
                if let Ok(data) = rx.recv() {
                    match data {
                        MarketData::Tick(tick) => {
                            let json = serde_json::to_string(&tick).unwrap();
                            info!("test {}", json);
                        },
                        _ => {
                            info!("ssss");
                        },
                    }
                }
            }
        });
        handler.join().unwrap();
        Ok(())
    }

}