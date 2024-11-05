use std::{net::TcpStream, sync::{atomic::{AtomicUsize, Ordering}, Arc, Mutex}, thread::{self, sleep}, time::{Duration, Instant}};
use tungstenite::{stream::MaybeTlsStream, Message};
use crate::tungstenite::{BinanceWebSocketClient, WebSocketState};
use std::error::Error;
type Conn = WebSocketState<MaybeTlsStream<TcpStream>>;
pub struct WssListeneKeyKeepalive {
    renew_interval: u32,
    new_interval: u32,
    url: String,
    new_block: Option<Arc<Mutex<Box<dyn Fn() -> Result<String, Box<dyn Error>>>>>>,
    renew_block: Option<Arc<Mutex<Box<dyn Fn(&str) -> Result<(), Box<dyn Error>> + Send + 'static>>>>,
    conn: Option<Conn>,
    listen_key: String,
    keepalive_ticket: Arc<AtomicUsize>,
    conn_ticket: Arc<AtomicUsize>,
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
            keepalive_ticket: Arc::new(AtomicUsize::new(0)),
            conn_ticket: Arc::new(AtomicUsize::new(0)),
        }
    }

    fn connect(&mut self, listen_key: &str) -> &Self {
        self.listen_key = listen_key.to_string();
        let ret = BinanceWebSocketClient::connect_with_url(format!("{}/{}", self.url.as_str(), listen_key).as_str());
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

    pub fn close(&mut self) {
        self.conn_ticket.fetch_add(1, Ordering::SeqCst);
    }

    pub fn new_listen_key<F: 'static>(mut self, block: F, new_interval: u32) -> Self 
        where F: Fn() -> Result<String, Box<dyn Error>> {
        self.new_block = Some(Arc::new(Mutex::new(Box::new(block))));
        self.new_interval = new_interval;
        self
    }

    pub fn renew_listen_key<F: 'static>(mut self, block: F, renew_interval: u32) -> Self 
        where F: Fn(&str) -> Result<(), Box<dyn Error>> + Send + 'static {
        self.renew_block = Some(Arc::new(Mutex::new(Box::new(block))));
        self.renew_interval = renew_interval;
        self
    }

    fn keepalive(&self) {
        if let Some(renew_block_ref) = self.renew_block.as_ref() {
            let renew_block_ref = renew_block_ref.clone();
            let renew_interval = self.renew_interval as u64;
            let listen_key = self.listen_key.clone();
            let ticket = self.keepalive_ticket.load(Ordering::SeqCst);
            let ticket_load = self.keepalive_ticket.clone();

            thread::spawn(move || {
                let block = renew_block_ref.lock().unwrap();
                let mut exit_flag = false;
                while exit_flag {
                    let now = Instant::now();
                    loop {
                        let ticket2 = ticket_load.load(Ordering::SeqCst);
                        if ticket != ticket2 {
                            exit_flag = true;
                            break;
                        }
                        sleep(Duration::from_secs(1));
                        if now.elapsed() > Duration::from_secs(renew_interval as u64 * 0.9 as u64) {
                            let ret = block(&listen_key);
                            if ret.is_ok() {
                                break;
                            }
                        }
                    }
                }
            });
        }
    }

    pub fn stream<F>(&mut self, block: &mut F, skip_error: bool) -> Result<(), Box<dyn Error>> 
        where F: FnMut(Message) -> Result<bool, Box<dyn Error>> {
        let ticket = self.conn_ticket.fetch_add(1, Ordering::SeqCst);
        loop {
            if ticket != self.conn_ticket.load(Ordering::SeqCst) {
                break;
            }
            if self.conn.is_none() {
                println!("Connecting...");
                self.keepalive_ticket.fetch_add(1, Ordering::SeqCst);
                if let Some(b) = self.new_block.as_ref() {
                    let ret = b.lock().unwrap()();
                    if let Ok(key) = ret {
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
                    if conn.as_mut().can_read() {
                        let ret = conn.as_mut().read();
                        match ret {
                            Ok(message) => {
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
                            Err(e) => {
                                println!("Error: {:?}", e);
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
        Ok(())
    }
}