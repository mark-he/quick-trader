use crate::websocket::Stream;
use strum::Display;

/// Diff. Depth Stream
///
/// Order book price and quantity depth updates used to locally manage an order book.

#[derive(Copy, Clone, Display)]
pub enum UpdateSpeed {
    #[strum(serialize = "100ms")]
    Faster,
    #[strum(serialize = "500ms")]
    Slower,
}

pub struct DiffDepthStream {
    symbol: String,
    update_speed: Option<UpdateSpeed>,
}

impl DiffDepthStream {
    pub fn new(symbol: &str) -> Self {
        Self {
            symbol: symbol.to_lowercase(),
            update_speed: None,
        }
    }
}

impl From<DiffDepthStream> for Stream {
    fn from(stream: DiffDepthStream) -> Stream {
        if let Some(s) = stream.update_speed {
            Stream::new(&format!("{}@depth@{}", stream.symbol, s))
        } else {
            Stream::new(&format!("{}@depth", stream.symbol))
        }
    }
}
