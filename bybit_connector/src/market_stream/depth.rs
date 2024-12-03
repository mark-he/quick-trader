
use crate::websocket::Stream;

pub struct DepthStream {
    symbol: String,
    depth: u32,
}

impl DepthStream {
    pub fn new(symbol: &str, depth: u32) -> Self {
        Self {
            symbol: symbol.to_owned(),
            depth: depth,
        }
    }
}

impl From<DepthStream> for Stream {
    fn from(stream: DepthStream) -> Stream {
        Stream::new(&format!("orderbook.{}.{}", stream.depth, stream.symbol))
    }
}
