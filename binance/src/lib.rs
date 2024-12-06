pub mod bn_market_server;
pub mod bn_trade_server;
pub mod model;
pub mod bn_sim_market_server;
pub mod bn_sim_trade_server;

pub fn enable_prod(enabled: bool) {
    binance_future_connector::config::enable_prod(enabled);
}