
use crate::websocket::Stream;

pub struct TickerStream {
    symbol: String,
}

impl TickerStream {
    pub fn new(symbol: &str) -> Self {
        Self {
            symbol: symbol.to_owned(),
        }
    }
}

impl From<TickerStream> for Stream {
    fn from(stream: TickerStream) -> Stream {
        Stream::new(&format!("tickers.{}", stream.symbol))
    }
}
