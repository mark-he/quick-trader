use std::{net::TcpStream, sync::Arc, thread, time::Duration};
use tungstenite::{stream::MaybeTlsStream, Message};
use crate::tungstenite::{BinanceWebSocketClient, WebSocketState};
use tokio::time::{sleep, Duration as TokioDuration};
use tokio::sync::Mutex;
use std::error::Error;
type Conn = WebSocketState<MaybeTlsStream<TcpStream>>;
pub struct WssListeneKeyKeepalive {
    renew_interval: u32,
    new_interval: u32,
    url: String,
    new_block: Option<Box<dyn Fn() -> Option<String>>>,
    renew_block: Option<Arc<Mutex<Box<dyn Fn(&str) + Send + 'static>>>>,
    conn: Option<Conn>,
    listen_key: String,
}

impl WssListeneKeyKeepalive {
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
            renew_interval: 0,
            new_interval: 0,
            new_block: None,
            renew_block: None,
            conn: None,
            listen_key: "".to_string(),
        }
    }

    fn connect(&mut self, listen_key: &str) -> &Self {
        self.listen_key = listen_key.to_string();
        let ret = BinanceWebSocketClient::connect_with_url(format!("{}/{}", self.url.as_str(), listen_key).as_str());
        if let Ok(conn) = ret {
            self.conn = Some(conn);
        } else {
            self.conn = None;
        }
        self
    }

    pub fn close(&mut self) {

    }

    pub fn new_listen_key<F: 'static>(mut self, block: F, new_interval: u32) -> Self 
        where F: Fn() -> Option<String> {
        self.new_block = Some(Box::new(block));
        self.new_interval = new_interval;
        self
    }

    pub fn renew_listen_key<F: 'static>(mut self, block: F, renew_interval: u32) -> Self 
        where F: Fn(&str) + Send + 'static {
        self.renew_block = Some(Arc::new(Mutex::new(Box::new(block))));
        self.renew_interval = renew_interval;
        self
    }

    fn keepalive(&self) {
        if self.renew_block.is_some() {
            let renew_block_ref = self.renew_block.as_ref().unwrap().clone();
            let duration = self.renew_interval as u64 * 0.75 as u64;
            let listen_key = self.listen_key.clone();
            
            let closure = async move {
                let block = renew_block_ref.lock().await;
                let delay = TokioDuration::from_secs(duration);
                sleep(delay).await;
                block(listen_key.as_str());
            };
            tokio::runtime::Runtime::new().unwrap().spawn(closure);
        }
    }

    pub fn stream<F>(&mut self, block: F, skip_error: bool) -> Result<(), Box<dyn Error>> 
        where F: Fn(Message) -> Result<bool, Box<dyn Error>> {
        loop {
            if self.conn.is_none() {
                if let Some(b) = self.new_block.as_ref() {
                    let listen_key = b();
                    if let Some(key) = listen_key {
                        self.connect(key.as_str());
                        if self.conn.is_some() {
                            self.keepalive();
                            continue;
                        }
                    }
                } 
                thread::sleep(Duration::from_secs(1));
            } else {
                let conn = self.conn.as_mut().unwrap();
                loop {
                    let ret = conn.as_mut().read();
                    let mut block_ret= Ok(true);
                    let mut reset_conn_flag = false;
                    match ret {
                        Ok(message) => {
                            match message {
                                Message::Close(_) => {
                                    reset_conn_flag = true;
                                },
                                _ => {
                                },
                            }
                            block_ret = block(message);
                        },
                        Err(e) => {
                            println!("Error: {:?}", e);
                            reset_conn_flag = true;
                        }
                    }

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

                    if reset_conn_flag {
                        self.conn = None;
                        break;
                    }
                }
            }
        }
    }
}