
use crate::websocket::Stream;

pub struct DepthStream {
    symbol: String,
    depth: String,
}

impl DepthStream {
    pub fn new(symbol: &str, depth: &str) -> Self {
        Self {
            symbol: symbol.to_lowercase(),
            depth: depth.to_lowercase(),
        }
    }
}

impl From<DepthStream> for Stream {
    fn from(stream: DepthStream) -> Stream {
        Stream::new(&format!("orderbook.{}.{}", stream.depth, stream.symbol))
    }
}
