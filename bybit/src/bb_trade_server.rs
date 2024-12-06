use std::{sync::{atomic::{AtomicUsize, Ordering}, Arc, Mutex, RwLock}, thread::JoinHandle};
use common::{error::AppError, msmc::Subscription, thread::{Handler, InteractiveThread, Rx}};
use serde_json::Value;
use bybit_connector::{
    account, enums::Category, http::Credentials, trade::{self as bb_trade, new_order::NewOrderRequest}, ureq::BybitHttpClient, websocket::Stream, wss_keepalive::WssKeepalive
};
use trade::trade_server::{Order, Position, TradeEvent, TradeServer, Wallet};
use tungstenite::Message;
use crate::model::*;
use log::*;

pub struct WssStream {
    subscription: Arc<Mutex<Subscription<TradeEvent>>>,
    handler: Option<Handler<()>>,
    connect_ticket: Arc<AtomicUsize>,
    server_ping: Arc<AtomicUsize>,
    pub credentials: Credentials,
}

impl WssStream {
    pub fn new(credentials: Credentials) -> Self {
        WssStream {
            subscription: Arc::new(Mutex::new(Subscription::top())),
            handler : None,
            connect_ticket: Arc::new(AtomicUsize::new(0)),
            server_ping: Arc::new(AtomicUsize::new(0)),
            credentials,
        }
    }

    pub fn cleanup(&mut self) {
        self.subscription = Arc::new(Mutex::new(Subscription::top()));
        self.server_ping = Arc::new(AtomicUsize::new(0));
        self.handler = None;
    }

    pub fn subscribe(&mut self) -> Subscription<TradeEvent> {
        self.subscription.lock().unwrap().subscribe()
    }

    pub fn connect(&mut self, _credentials: Credentials) {
        let connect_ticket = self.connect_ticket.fetch_add(1, Ordering::SeqCst);
        let connect_ticket_ref = self.connect_ticket.clone();
        let server_ping_ref = self.server_ping.clone();
        let subscription_ref = self.subscription.clone();
        let credentials = self.credentials.clone();
        let closure = move |_rx: Rx<String>| {
            let subscription = subscription_ref.lock().unwrap();
            let mut keepalive = WssKeepalive::new(&format!("{}/v5/private", bybit_connector::config::wss_api())).credentials(credentials).prepare(|conn| {
                conn.subscribe(vec![
                    &Stream::new("order"), 
                    &Stream::new("position"), 
                    &Stream::new("wallet")
                    ]);
            });

            let _ = keepalive.stream(&mut |message| {
                if connect_ticket != connect_ticket_ref.load(Ordering::SeqCst) - 1 {
                    return Ok(true);
                }
                match message {
                    Message::Text(string_data) => {
                        let json_value: Value = serde_json::from_str(&string_data).unwrap();
                        let topic = json_value.get("topic");
                        if let Some(topic_value) = topic {
                            let vs: Vec<&str> = topic_value.as_str().unwrap().split('.').collect();
                            let event = vs[0];
                            match event {
                                "position" => {
                                    let position_update: PositionData = serde_json::from_str(&string_data).map_err(|e| Box::new(e))?;
                                    for p in position_update.data {
                                        let position = Position {
                                            symbol: p.symbol,
                                            position_side: p.position_idx.to_string(),
                                            side: p.side.clone(),
                                            amount: p.size,
                                            cost: p.entry_price,
                                            ..Default::default()
                                        };
                                        subscription.send(&TradeEvent::PositionUpdate(position));
                                    }
                                },
                                "wallet" => {
                                    let wallet_update: WalletData = serde_json::from_str(&string_data).map_err(|e| Box::new(e))?;
                                    for w in wallet_update.data {
                                        for c in w.coin {
                                            let wallet = Wallet {
                                                asset: c.coin.clone(),
                                                balance: c.wallet_balance,
                                                available_balance: 0 as f64,
                                            };
                                            subscription.send(&TradeEvent::AccountUpdate(wallet));
                                        }
                                    }
                                },
                                "order" => {
                                    let order_update: OrderData = serde_json::from_str(&string_data).map_err(|e| Box::new(e))?;
                                    for o in order_update.data {
                                        let order = Order {
                                            order_id: o.order_id.clone(),
                                            client_order_id: o.order_link_id.clone(),
                                            order_type: o.order_type,
                                            symbol: o.symbol.clone(),
                                            status: o.order_status.clone(),
                                            traded: o.cum_exec_qty,
                                            total: o.qty,
                                            side: o.side.clone(),
                                            message: o.reject_reason.clone(),
                                            timestamp: o.created_time as u64,
                                            offset: if o.reduce_only { "CLOSE".to_string() } else {"OPEN".to_string()},
                                            ..Default::default()
                                        };
                                        subscription.send(&TradeEvent::OrderUpdate(order));
                                    }
                                },
                                _ => {
                                    debug!("Received other event: {}", string_data);
                                },
                            }
                        } else {
                            warn!("Received unknown event: {}", string_data);
                        }
                    },
                    Message::Ping(data) => {
                        let string_data = String::from_utf8(data)?;
                        server_ping_ref.store(string_data.parse::<usize>()?, Ordering::SeqCst);
                    },
                    _ => {
                        warn!("Unexpected message: {:?}", message);
                    },
                }
                Ok(true)
            }, true);
        };

        let handler = InteractiveThread::spawn(closure);
        self.handler = Some(handler);
    }

    fn close(&self) {
        self.connect_ticket.fetch_add(1, Ordering::SeqCst);
    }
}

pub trait BbTradeServerTrait : TradeServer<
        OrderRequest = NewOrderRequest,
        CancelOrderRequest = String,
        SymbolConfig = SymbolConfig,
        SymbolInfo = SymbolInfo,
        Symbol = String,
        > {}

impl BbTradeServerTrait for BbTradeServer {}

pub struct BbTradeServer {
    pub config: BbTradeConfig,
    pub credentials: Credentials,
    pub wss_stream: WssStream,
    pub positions: Arc<RwLock<Vec<Position>>>,
    pub wallets: Arc<RwLock<Vec<Wallet>>>,
    pub handler: Option<JoinHandle<()>>,
    pub subscription: Arc<Mutex<Subscription<TradeEvent>>>,
}

impl BbTradeServer {
    pub fn new(config: BbTradeConfig) -> Self {
        let credentials = Credentials::from_hmac(config.api_key.clone(), config.api_secret.clone());
        BbTradeServer {
            credentials: credentials.clone(),
            config,
            wss_stream: WssStream::new(credentials.clone()),
            positions: Arc::new(RwLock::new(Vec::new())),
            wallets: Arc::new(RwLock::new(Vec::new())),
            handler: None,
            subscription: Arc::new(Mutex::new(Subscription::top())),
        }
    }
    
    fn monitor_account_positions(&mut self) {
        let wallets_ref = self.wallets.clone();
        let positions_ref = self.positions.clone();

        let handler = self.subscription.lock().unwrap().stream(move |event| {
            if let Some(e) = event {
                match e {
                    TradeEvent::PositionUpdate(a) => {
                        let mut positions = positions_ref.write().unwrap();
                        let mut found = false;
                        let mut changed = false;
                        info!("POSITION UPDATE {:?}", a);
                        for p in positions.iter_mut() {
                            if p.symbol == a.symbol && p.position_side == a.position_side {
                                found = true;
                                if p != a {
                                    changed = true;
                                    p.cost = a.cost;
                                    p.symbol = a.symbol.clone();
                                    p.amount = a.amount;
                                    p.side = a.side.clone();
                                    info!("POSITION UPDATE FOUND !!! {:?}", a);
                                }
                                break;
                            }
                        }
                        if !found {
                            if a.side != "" {
                                positions.push(a.clone());
                                changed = true;
                            }
                        }
                        info!("POSITION UPDATE AFTER !!! {:?}", positions);
                        return Ok(changed);
                    },
                    TradeEvent::AccountUpdate(a) => {
                        let mut wallets = wallets_ref.write().unwrap();
                        let mut found = false;
                        for p in wallets.iter_mut() {
                            if p.asset == a.asset {
                                p.balance = a.balance;
                                p.available_balance = a.available_balance;
                                found = true;
                                break;
                            }
                        }
                        if !found {
                            wallets.push(a.clone());
                        }
                    },
                    _ => {},
                }
            }
            Ok(true)
        });
        self.handler = Some(handler);
    }

    fn init_account(&self) -> Result<(), AppError> {
        let client = BybitHttpClient::default().credentials(self.credentials.clone());
        let request = bb_trade::set_margin_mode(&self.config.margin_mode);
        let _ = get_resp_result::<Ignore>(client.send(request), vec![], true)?;
        let _ = get_resp_result::<Ignore>(client.send(bb_trade::position_side(Category::Linear, self.config.position_side).coin(&self.config.settle_coin)), vec![], true)?;
        
        Ok(())
    }

    fn init_account_positions(&self) -> Result<(), AppError> {
        let client = BybitHttpClient::default().credentials(self.credentials.clone());
        let data = get_resp_result::<AccountQueryResp>(client.send(account::account(account::account::AccountType::Unified)), vec![], false)?;
        if let Some(account_resp) = data {
            for a in account_resp.list {
                for w in a.coin {
                    let wallet = Wallet {
                        asset: w.coin.clone(),
                        balance: w.wallet_balance,
                        available_balance: w.available_to_withdraw,
                    };
                    self.wallets.write().unwrap().push(wallet);
                }
            }
        }
        
        let data = get_resp_result::<PositionQueryResp>(client.send(account::position(Category::Linear).settle_coin(&self.config.settle_coin)), vec![], false)?;
        if let Some(position_resp) = data {
            for a in position_resp.list {
                let position = Position {
                    symbol: a.symbol.clone(),
                    position_side: a.position_idx.to_string(),
                    side: a.side.to_string(),
                    amount: a.size,
                    cost: a.avg_price,
                    ..Default::default()
                };
                self.positions.write().unwrap().push(position);
            }
        }
        Ok(())
    }
}

pub type BbTradeServerType = dyn TradeServer<
            OrderRequest = NewOrderRequest,
            CancelOrderRequest = String,
            SymbolConfig = SymbolConfig,
            SymbolInfo = SymbolInfo,
            Symbol = String,
            >;

impl TradeServer for BbTradeServer {
    type OrderRequest = NewOrderRequest;
    type CancelOrderRequest = String;
    type SymbolConfig = SymbolConfig;
    type SymbolInfo = SymbolInfo;
    type Symbol = String;
    
    fn init(&mut self) -> Result<(), AppError> {
        self.wss_stream.cleanup();
        self.init_account()?;
        self.init_account_positions()?;
        Ok(())
    }

    fn start(&mut self) -> Result<Subscription<TradeEvent>, AppError> {
        let mut sub = self.wss_stream.subscribe();
        let ext_sub = sub.subscribe();
        self.subscription = Arc::new(Mutex::new(sub));

        self.monitor_account_positions();

        let credentials = self.credentials.clone();
        self.wss_stream.connect(credentials);
        Ok(ext_sub)
    }

    fn new_order(&mut self, _symbol: String, request : NewOrderRequest) -> Result<(), AppError> {
        let client = BybitHttpClient::default().credentials(self.credentials.clone());
        let _ = get_resp_result::<Ignore>(client.send(request), vec![], true)?;
        Ok(())
    }

    fn cancel_order(&mut self, symbol: String, request: String) -> Result<(), AppError> {
        let client = BybitHttpClient::default().credentials(self.credentials.clone());
        let request = bb_trade::cancel_order(Category::Linear, &symbol).order_link_id(&request);
        info!("Cancel Order {:?}", request);
        let _ = get_resp_result::<Ignore>(client.send(request), vec![], true)?;
        Ok(())
    }

    fn cancel_orders(&mut self, symbol: String) -> Result<(), AppError> {
        let client = BybitHttpClient::default().credentials(self.credentials.clone());
        let request = bb_trade::cancel_orders(Category::Linear, &symbol);
        let _ = get_resp_result::<Ignore>(client.send(request), vec![], true)?;
        Ok(())
    }

    fn get_positions(&self, symbol: String) -> Result<Vec<Position>, AppError> {
        let positions = self.positions.read().unwrap();
        let mut ret = vec![];
        for position in positions.iter() {
            if position.symbol == symbol && position.amount > 0.0 {
                ret.push(position.clone());
            }
        }
        Ok(ret)
    }

    fn get_account(&self, account_id: &str) -> Result<Option<Wallet>, AppError>{
        let wallets = self.wallets.read().unwrap();
        let mut ret = None;
        for asset in wallets.iter() {
            if account_id == asset.asset && asset.balance > 0.0 {
                ret = Some(asset.clone());
                break;
            }
        }
        Ok(ret)
    }
    
    fn init_symbol(&self, symbol: String, config: Self::SymbolConfig) -> Result<SymbolInfo, AppError> {
        let client = BybitHttpClient::default().credentials(self.credentials.clone());

        let request = bb_trade::leverage(Category::Linear, &symbol, &config.leverage.to_string(), &config.leverage.to_string());
        let _ = get_resp_result::<Ignore>(client.send(request), vec![110043], true)?;

        
        let symbol_info = SymbolInfo {
            symbol: symbol.to_string(),
            leverage: config.leverage,
            margin_type: config.margin_type.to_string(),
            dual_position_side: self.config.position_side.to_string(),
        };

        Ok(symbol_info)
    }

    fn close(&self) {
        self.wss_stream.close();
    }
}
