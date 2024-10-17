use std::net::TcpStream;
use tungstenite::{connect, stream::MaybeTlsStream, Error, Message, WebSocket};
use crate::tungstenite::{BinanceWebSocketClient, WebSocketState};

pub struct WssKeepaliveApi {
    url: String,
    prepare_block: Option<Box<dyn Fn(&WebSocketState<MaybeTlsStream<TcpStream>>)>>,
    conn: Option<WebSocketState<MaybeTlsStream<TcpStream>>>,
}

impl WssKeepaliveApi {
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
            prepare_block: None,
            conn: None,
        }
    }

    pub fn prepare<'a, F>(mut self, block: F) -> Self 
        where F: Fn(&'a WebSocketState<MaybeTlsStream<TcpStream>>) {
        self.prepare_block = Some(Box::new(block));
        self
    }

    pub fn stream<F>(&self, f: F) 
        where F: Fn(Message) {

    }

    pub fn connect(&mut self) -> &Self {
        let ret = BinanceWebSocketClient::connect_with_url(self.url.as_str());
        if let Ok(conn) = ret {
            self.conn = Some(conn);
        } else {
            self.conn = None;
        }
        self
    }


    pub fn ready<F>(&self, f: F) -> Self 
        where F : FnMut(&Option<T>){

    }
}