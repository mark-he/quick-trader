
use crate::websocket::Stream;

pub struct WalletStream {
}

impl WalletStream {
    pub fn new() -> Self {
        Self {
        }
    }
}

impl From<WalletStream> for Stream {
    fn from(_stream: WalletStream) -> Stream {
        Stream::new("wallet")
    }
}
