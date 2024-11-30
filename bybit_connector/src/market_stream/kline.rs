
use crate::{enums::KlineInterval, websocket::Stream};

pub struct KlineStream {
    symbol: String,
    interval: KlineInterval,
}

impl KlineStream {
    pub fn new(symbol: &str, interval: KlineInterval) -> Self {
        Self {
            symbol: symbol.to_lowercase(),
            interval,
        }
    }
}

impl From<KlineStream> for Stream {
    /// kline.{interval}.{symbol} e.g., kline.30.BTCUSDT
    fn from(stream: KlineStream) -> Stream {
        Stream::new(&format!("kline.{}.{}", stream.interval, stream.symbol))
    }
}
