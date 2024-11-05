use std::{net::TcpStream, thread, time::Duration};
use tungstenite::{stream::MaybeTlsStream, Message};
use crate::tungstenite::{BinanceWebSocketClient, WebSocketState};
use std::error::Error;

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
        let ret = BinanceWebSocketClient::connect_with_url(&self.url);
        match ret {
            Ok(conn) => {
                self.conn = Some(conn);
            },
            Err(e) => {
                println!("CONNECTED FAILED!: {:?}", e);
                self.conn = None;
            },
        }
        self
    }

    pub fn prepare<F: 'static>(mut self, block: F) -> Self 
        where F: Fn(&mut Conn) {
        self.prepare_block = Some(Box::new(block));
        self
    }

    pub fn stream<F>(&mut self, block: F, skip_error: bool) -> Result<(), Box<dyn Error>>
        where F: Fn(Message) -> Result<bool, Box<dyn Error>> {
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
                let conn = self.conn.as_mut().unwrap();

                loop {
                    if conn.as_mut().can_read() {
                        let ret = conn.as_mut().read();
                        match ret {
                            Ok(message) => {
                                match message {
                                    _ => {
                                        let block_ret = block(message);
                                        match block_ret {
                                            Ok(continue_flag) => {
                                                if !continue_flag {
                                                    return Ok(());
                                                }
                                            },
                                            Err(e) => {
                                                println!("Error: {:?}", e);
                                                if !skip_error {
                                                    return Err(e);
                                                }
                                            },
                                        }
                                    },
                                }
                            },
                            Err(e) => {
                                println!("Error data {:?}", e);
                            }
                        }
                    } else {
                        println!("Can't read data");
                        self.conn = None;
                        break;
                    }
                }
            }
        }
    }
}