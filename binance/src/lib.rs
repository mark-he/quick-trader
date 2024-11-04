pub mod bn_market_server;
pub mod bn_trade_server;
pub mod model;

pub fn enable_prod(enabled: bool) {
    binance_future_connector::config::enable_prod(enabled);
}