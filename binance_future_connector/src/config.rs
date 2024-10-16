
#[cfg(not(feature = "prod"))]
pub const REST_API: &str = "https://testnet.binancefuture.com";

#[cfg(not(feature = "prod"))]
pub const WSS_API: &str = "wss://fstream.binancefuture.com/ws";

#[cfg(feature = "prod")]
pub const REST_API: &str = "https://fapi.binance.com";

#[cfg(feature = "prod")]
pub const WSS_API: &str = "wss://fstream.binance.com/ws";





