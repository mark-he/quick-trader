pub mod spi;
pub mod context;
pub mod c_model;

#[cfg(test)]
mod tests {
    use ctp::{ctp_market_server::CtpMarketServer, ctp_trade_server::CtpTradeServer, model::Config};
    use std::os::raw::*;
    use std::ffi::CString;
    use std::str::FromStr;
    use common::c::*;
    use market::market_server::{KLine, MarketData};
    use std::thread;
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
    fn test_market_kline() {
        init_gateweay();
        let gateway_ref = context::get_market_gateway();
        let mut gateway = gateway_ref.lock().unwrap();
    
        let rx  = gateway.subscribe_kline("m2501", "1m");
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
                            info!("{}", json);
                        },
                        _ => {},
                    }
                }
            }
        });
        handler.join().unwrap();
    }

    #[test]
    fn test_market_tick() {
        init_gateweay();

        let gateway_ref = context::get_market_gateway();
        let mut gateway = gateway_ref.lock().unwrap();
    
        let rx  = gateway.subscribe_tick("m2501");
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
    }
}