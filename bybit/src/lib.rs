pub mod bb_market_server;
pub mod bb_trade_server;
pub mod model;
pub mod bb_sim_market_server;
pub mod bb_sim_trade_server;

pub fn enable_prod(enabled: bool) {
    bybit_connector::config::enable_prod(enabled);
}