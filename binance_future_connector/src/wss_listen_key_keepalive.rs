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
    stream_ticket: Arc<AtomicUsize>,
    conn_instant: Instant,
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
            conn_instant: Instant::now(),
            listen_key: "".to_string(),
            keepalive_ticket: Arc::new(AtomicUsize::new(0)),
            stream_ticket: Arc::new(AtomicUsize::new(0)),
        }
    }

    fn connect(&mut self, listen_key: &str) -> &Self {
        self.listen_key = listen_key.to_string();
        let ret = BinanceWebSocketClient::connect_with_url(format!("{}/{}", self.url.as_str(), listen_key).as_str());
        match ret {
            Ok(conn) => {
                self.conn = Some(conn);
                self.conn_instant = Instant::now();
            },
            Err(e) => {
                println!("CONNECTED FAILED!: {:?}", e);
                self.conn = None;
            },
        }
        self
    }

    pub fn close(&mut self) {
        self.stream_ticket.fetch_add(1, Ordering::SeqCst);
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
        let keepalive_ticket = self.keepalive_ticket.fetch_add(1, Ordering::SeqCst);
        let keepalive_ticket_ref = self.keepalive_ticket.clone();

        if let Some(renew_block_ref) = self.renew_block.as_ref() {
            let renew_block_ref = renew_block_ref.clone();
            let renew_interval = self.renew_interval as u64;
            let listen_key = self.listen_key.clone();
            thread::spawn(move || {
                let block = renew_block_ref.lock().unwrap();
                let mut exit_flag = true;
                let mut renew = Instant::now();
                while exit_flag {
                    loop {
                        if keepalive_ticket != keepalive_ticket_ref.load(Ordering::SeqCst) - 1{
                            println!("Ticket exit wss_listen_key_keepalive");
                            exit_flag = true;
                            break;
                        }
                        sleep(Duration::from_secs(1));
                        if renew.elapsed().as_secs() as f64 >= (renew_interval as f64 * 0.9) {
                            let ret = block(&listen_key);
                            if ret.is_ok() {
                                renew = Instant::now();
                                println!("Renew >>>> {:?}", ret.unwrap());
                                break;
                            } else {
                                println!("Error >>>> {:?}", ret.unwrap_err());
                            }
                        }
                    }
                }
            });
        }
    }

    pub fn stream<F>(&mut self, block: &mut F, skip_error: bool) -> Result<(), Box<dyn Error>> 
        where F: FnMut(Message) -> Result<bool, Box<dyn Error>> {
        let stream_ticket = self.stream_ticket.fetch_add(1, Ordering::SeqCst);
        loop {
            if stream_ticket != self.stream_ticket.load(Ordering::SeqCst) - 1 {
                break;
            }
            if self.conn.is_none() {
                println!("Connecting Listen Key...");
                if let Some(b) = self.new_block.as_ref() {
                    let ret = b.lock().unwrap()();
                    if let Ok(key) = ret {
                        self.connect(key.as_str());
                        if self.conn.is_some() {
                            self.keepalive();
                            continue;
                        }
                        println!("Renconnect >>>> {:?}", key);
                    } else {
                        println!("Error >>>> {:?}", ret.unwrap_err());
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
                                let block_ret = block(message.clone());
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

                                if let Message::Ping(_) = message {                    
                                    println!("Keepalive at connection: {:?}, {:?}", self.conn_instant.elapsed().as_secs(), self.new_interval as f64 * 0.9);
                                    if self.conn_instant.elapsed().as_secs() as f64 >= (self.new_interval as f64 * 0.9) {
                                        self.conn = None;
                                        break;
                                    }
                                }
                            },
                            
                            Err(e) => {
                                println!("Error: {:?}", e);
                            }
                        }
                    } else {
                        println!("Connection disconnected.");
                        self.conn = None;
                        break;
                    }
                }
            }
        }
        Ok(())
    }
}