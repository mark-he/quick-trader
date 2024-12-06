use std::{net::TcpStream, sync::{atomic::{AtomicUsize, Ordering}, Arc}, thread::{self, sleep}, time::{Duration, Instant, SystemTime, UNIX_EPOCH}};
use log::info;
use tungstenite::{stream::MaybeTlsStream, Message};
use crate::{http::Credentials, tungstenite::{BybitWebSocketClient, WebSocketState}};
use std::error::Error;

type Conn = WebSocketState<MaybeTlsStream<TcpStream>>;
pub struct WssKeepalive {
    url: String,
    prepare_block: Option<Box<dyn Fn(&mut Conn)>>,
    conn: Option<Conn>,
    stream_ticket: Arc<AtomicUsize>,
    credentials: Option<Credentials>,
    timestamp_delta: i64,
}

impl WssKeepalive {
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
            prepare_block: None,
            conn: None,
            timestamp_delta: 0,
            stream_ticket: Arc::new(AtomicUsize::new(0)),
            credentials: None,
        }
    }

    pub fn credentials(mut self, credentials: Credentials) -> Self {
        self.credentials = Some(credentials);
        self
    }

    pub fn timestamp_delta(mut self, timestamp_delta: i64) -> Self {
        self.timestamp_delta = timestamp_delta;
        self
    }

    fn connect(&mut self) -> &Self {
        let ret = BybitWebSocketClient::connect_with_url(&self.url);
        match ret {
            Ok(conn) => {
                self.conn = Some(conn);
            },
            Err(e) => {
                println!("Connection failed. {:?}", e);
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

    pub fn close(&mut self) {
        self.stream_ticket.fetch_add(1, Ordering::SeqCst);
    }
    
    #[allow(unused_assignments)]
    pub fn stream<F>(&mut self, block: &mut F, skip_error: bool) -> Result<(), Box<dyn Error>>
        where F: FnMut(Message) -> Result<bool, Box<dyn Error>> {
        let stream_ticket = self.stream_ticket.fetch_add(1, Ordering::SeqCst);
        loop {
            if stream_ticket != self.stream_ticket.load(Ordering::SeqCst) - 1 {
                println!("Ticket exit wss_keepalive");
                break;
            }
            if self.conn.is_none() {
                self.connect();
                if self.conn.is_some() {
                    let mut_conn = self.conn.as_mut().unwrap();
                    if let Some(Credentials { api_key, signature }) = &self.credentials {
                        let mut timestamp = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .expect("Clock may have gone backwards")
                        .as_millis();
                        timestamp -= self.timestamp_delta as u128;
                        let payload = format!("GET/realtime{}", timestamp + 24 * 3600 * 1000);
                        
                        let signature = crate::utils::sign(
                            &payload,
                            signature,
                        )?;
            
                        let message = format!("{{\"op\": \"auth\",\"args\": [\"{}\",{},\"{}\"]}}", api_key, timestamp, signature);
                        info!("payload = {}, message = {}", payload, message);
                        mut_conn.as_mut().send(Message::Text(message))?;
                    }
                    if let Some(b) = self.prepare_block.as_ref() {
                        b(mut_conn);
                    }
                } else {
                    thread::sleep(Duration::from_secs(1));
                }
            } else {
                let conn = self.conn.as_mut().unwrap();

                let mut heartbeat = Instant::now();
                let mut trigger_time = 20;
                loop {
                    sleep(Duration::from_millis(10));
                    if heartbeat.elapsed().as_secs() >= trigger_time {
                        let ret = conn.as_mut().send(Message::Text("{\"op\": \"ping\"}".to_string()));
                        if ret.is_ok() {
                            println!("Heartbeat sent >>>>");
                            heartbeat = Instant::now();
                            trigger_time = 20;
                            break;
                        } else {
                            trigger_time = trigger_time + 2;
                            println!("Heartbeat Error >>>> {:?}", ret.unwrap_err());
                        }
                    }
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