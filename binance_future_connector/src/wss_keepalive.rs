use std::{net::TcpStream, thread, time::Duration};
use tungstenite::{stream::MaybeTlsStream, Message};
use crate::tungstenite::{BinanceWebSocketClient, WebSocketState};

type Conn = WebSocketState<MaybeTlsStream<TcpStream>>;
pub struct WssKeepalive {
    url: String,
    prepare_block: Option<Box<dyn Fn(&mut Conn)>>,
    conn: Option<Conn>,
}

impl WssKeepalive {
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
            prepare_block: None,
            conn: None,
        }
    }

    fn connect(&mut self) -> &Self {
        let ret = BinanceWebSocketClient::connect_with_url(self.url.as_str());
        if let Ok(conn) = ret {
            self.conn = Some(conn);
        } else {
            self.conn = None;
        }
        self
    }

    pub fn prepare<F: 'static>(mut self, block: F) -> Self 
        where F: Fn(&mut Conn) {
        self.prepare_block = Some(Box::new(block));
        self
    }

    pub fn stream<F>(&mut self, block: F) 
        where F: Fn(Message) -> bool {
        loop {
            if self.conn.is_none() {
                self.connect();
                if self.conn.is_some() {
                    if let Some(b) = self.prepare_block.as_ref() {
                        b(self.conn.as_mut().unwrap());
                    }
                } else {
                    thread::sleep(Duration::from_secs(1));
                }
            } else {
                let mut exit_flag = false;
                let conn = self.conn.as_mut().unwrap();

                loop {
                    let ret = conn.as_mut().read();
                    match ret {
                        Ok(message) => {
                            match message {
                                Message::Close(_) => {
                                    self.conn = None;
                                    exit_flag = !block(message);
                                    break;
                                },
                                _ => {
                                    exit_flag = !block(message);
                                    if exit_flag {
                                        break;
                                    }
                                },
                            }
                        },
                        Err(_) => {
                            self.conn = None;
                            break;
                        }
                    }
                }

                if exit_flag {
                    break;
                }
            }
        }
    }
}