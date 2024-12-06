use std::{sync::{atomic::{AtomicUsize, Ordering}, Arc, Mutex, RwLock}, thread::JoinHandle};
use common::{error::AppError, msmc::Subscription, thread::{Handler, InteractiveThread, Rx}};
use serde_json::Value;
use binance_future_connector::{
    account, http::Credentials, market as bn_market, trade::{self as bn_trade, enums::Side, new_order::NewOrderRequest}, ureq::BinanceHttpClient, user_data_stream, wss_listen_key_keepalive::WssListeneKeyKeepalive
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
}

impl WssStream {
    pub fn new() -> Self {
        WssStream {
            subscription: Arc::new(Mutex::new(Subscription::top())),
            handler : None,
            connect_ticket: Arc::new(AtomicUsize::new(0)),
            server_ping: Arc::new(AtomicUsize::new(0)),
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

    pub fn connect(&mut self, credentials: Credentials) {
        let connect_ticket = self.connect_ticket.fetch_add(1, Ordering::SeqCst);
        let connect_ticket_ref = self.connect_ticket.clone();
        let server_ping_ref = self.server_ping.clone();
        let subscription_ref = self.subscription.clone();

        let closure = move |_rx: Rx<String>| {
            let subscription = subscription_ref.lock().unwrap();
            let credentials2 = credentials.clone();
            let mut keepalive = WssListeneKeyKeepalive::new(&binance_future_connector::config::wss_api()).new_listen_key( move || {
                let client = BinanceHttpClient::default().credentials(credentials.clone());
                let request = user_data_stream::new_listen_key();
                let string_data = get_resp_result(client.send(request), vec![])?;

                let json_value: Value = serde_json::from_str(&string_data).unwrap();
                if let Some(key) = json_value.get("listenKey") {
                    Ok(key.as_str().unwrap().to_string())
                } else {
                    Err(Box::new(AppError::new(-200, format!("{:?}", "listenKey not found").as_str())))
                }
            }).renew_listen_key( move |listen_key| {
                let client = BinanceHttpClient::default().credentials(credentials2.clone());
                let request = user_data_stream::renew_listen_key(listen_key);
                let _ = get_resp_result(client.send(request), vec![])?;
                Ok(())
            }, 3000);

            let _ = keepalive.stream(&mut |message| {
                if connect_ticket != connect_ticket_ref.load(Ordering::SeqCst) - 1 {
                    return Ok(true);
                }
                match message {
                    Message::Text(string_data) => {
                        let json_value: Value = serde_json::from_str(&string_data).unwrap();

                        match json_value.get("e") {
                            Some(event_type) => {
                                let event = event_type.as_str().unwrap();
                                match event {
                                    "ACCOUNT_UPDATE" => {
                                        let account_update_event: AccountUpdateEvent = serde_json::from_str(&string_data).map_err(|e| Box::new(e))?;
                                        for w in account_update_event.update_data.balances {
                                            let wallet = Wallet {
                                                asset: w.asset.clone(),
                                                balance: w.cross_wallet_balance,
                                                available_balance: w.wallet_balance,
                                            };
                                            subscription.send(&TradeEvent::AccountUpdate(wallet));
                                        }

                                        for p in account_update_event.update_data.positions {
                                            let position = Position {
                                                symbol: p.symbol.clone(),
                                                position_side: p.position_side.clone(),
                                                side: if p.position_amount > 0.0 {Side::Buy.to_string()} else {Side::Sell.to_string()} ,
                                                amount: p.position_amount.abs(),
                                                cost: p.entry_price,
                                                ..Default::default()
                                            };
                                            subscription.send(&TradeEvent::PositionUpdate(position));
                                        }
                                    },
                                    "ORDER_TRADE_UPDATE" => {
                                        let order_trade_update_event= serde_json::from_str::<OrderTradeUpdateEvent>(&string_data).map_err(|e| Box::new(e))?;
                                        let order = Order {
                                            order_id: order_trade_update_event.order.order_id.to_string(),
                                            client_order_id: order_trade_update_event.order.client_order_id.clone(),
                                            symbol: order_trade_update_event.order.symbol.clone(),
                                            status: order_trade_update_event.order.order_status.clone(),
                                            traded: order_trade_update_event.order.order_filled_accumulated_quantity,
                                            total: order_trade_update_event.order.original_quantity,
                                            side: order_trade_update_event.order.side.clone(),
                                            timestamp: order_trade_update_event.order.order_trade_time,
                                            offset: if order_trade_update_event.order.is_reduce_only { "CLOSE".to_string() } else { "OPEN".to_string() },
                                            ..Default::default()
                                        };
                                        subscription.send(&TradeEvent::OrderUpdate(order));
                                    },
                                    _ => {
                                        debug!("Received other event: {}", string_data);
                                    },
                                }
                            },
                            None => {
                                warn!("Received unknown event: {}", string_data);
                            },
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

pub struct BnTradeServer {
    pub config: BnTradeConfig,
    pub credentials: Credentials,
    pub wss_stream: WssStream,
    pub positions: Arc<RwLock<Vec<Position>>>,
    pub wallets: Arc<RwLock<Vec<Wallet>>>,
    pub exchange_info: Option<ExchangeInfoQueryResp>,
    pub handler: Option<JoinHandle<()>>,
    pub subscription: Arc<Mutex<Subscription<TradeEvent>>>,
}

impl BnTradeServer {
    pub fn new(config: BnTradeConfig) -> Self {
        BnTradeServer {
            credentials: Credentials::from_hmac(config.api_key.clone(), config.api_secret.clone()),
            config,
            wss_stream: WssStream::new(),
            positions: Arc::new(RwLock::new(Vec::new())),
            wallets: Arc::new(RwLock::new(Vec::new())),
            exchange_info: None,
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
                        for p in positions.iter_mut() {
                            if p.symbol == a.symbol && p.position_side == a.position_side {
                                p.cost = a.cost;
                                p.symbol = a.symbol.clone();
                                p.side = a.side.clone();
                                p.amount = a.amount;
                                found = true;
                                break;
                            }
                        }
                        if !found {
                            positions.push(a.clone());
                        }
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
        let client = BinanceHttpClient::default().credentials(self.credentials.clone());
        let _ = get_resp_result(client.send(bn_trade::multi_assets_margin(self.config.multi_assets_margin)), vec![-4171])?;
        let _ = get_resp_result(client.send(bn_trade::position_side(self.config.dual_position_side)), vec![-4059])?;
        Ok(())
    }

    fn init_exchange(&mut self) -> Result<(), AppError> {
        let client = BinanceHttpClient::default().credentials(self.credentials.clone());
        let data = get_resp_result(client.send(bn_market::exchange_info()), vec![])?;
        let exchange_info: ExchangeInfoQueryResp = serde_json::from_str(&data).map_err(|e| AppError::new(-200, format!("{:?}", e).as_str()))?;
        self.exchange_info = Some(exchange_info);
        Ok(())
    }

    fn init_account_positions(&self) -> Result<(), AppError> {
        let client = BinanceHttpClient::default().credentials(self.credentials.clone());
        let data = get_resp_result(client.send(account::account()), vec![])?;
        let account_resp: AccountQueryResp = serde_json::from_str(&data).map_err(|e| AppError::new(-200, format!("{:?}", e).as_str()))?;

        for a in account_resp.assets {
            let wallet = Wallet {
                asset: a.asset.clone(),
                balance: a.cross_wallet_balance,
                available_balance: a.available_balance,
            };
            self.wallets.write().unwrap().push(wallet);
        }

        for a in account_resp.positions {
            let position = Position {
                symbol: a.symbol.clone(),
                position_side: a.position_side.to_string(),
                side: if a.position_amt > 0.0 {Side::Buy.to_string()} else {Side::Sell.to_string()},
                amount: a.position_amt.abs(),
                cost: a.entry_price,
                ..Default::default()
            };
            self.positions.write().unwrap().push(position);
        }
        Ok(())
    }

}

impl TradeServer for BnTradeServer {
    type OrderRequest = NewOrderRequest;
    type CancelOrderRequest = String;
    type SymbolConfig = SymbolConfig;
    type SymbolInfo = SymbolInfo;
    type Symbol = String;
    
    fn init(&mut self) -> Result<(), AppError> {
        self.wss_stream.cleanup();
        self.init_exchange()?;
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
        let client = BinanceHttpClient::default().credentials(self.credentials.clone());
        let _ = get_resp_result(client.send(request), vec![])?;
        Ok(())
    }

    fn cancel_order(&mut self, symbol: String, request: String) -> Result<(), AppError> {
        let client = BinanceHttpClient::default().credentials(self.credentials.clone());
        let request = bn_trade::cancel_order(&symbol).orig_client_order_id(&request);
        info!("Cancel Order {:?}", request);
        let _ = get_resp_result(client.send(request), vec![])?;
        Ok(())
    }

    fn cancel_orders(&mut self, symbol: String) -> Result<(), AppError> {
        let client = BinanceHttpClient::default().credentials(self.credentials.clone());
        let request = bn_trade::cancel_open_orders(&symbol);
        let _ = get_resp_result(client.send(request), vec![])?;
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

    fn get_account(&self, account_id: &str) -> Result<Option<Wallet>, AppError> {
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
        let client = BinanceHttpClient::default().credentials(self.credentials.clone());

        let request = bn_trade::margin_type(&symbol, config.margin_type);
        let _ = get_resp_result(client.send(request), vec![-4046])?;

        let request = bn_trade::leverage(&symbol, config.leverage);
        let _ = get_resp_result(client.send(request), vec![])?;

        let request = account::leverage_bracket().symbol(&symbol);
        let data = get_resp_result(client.send(request), vec![])?;

        let leverage_brackets: Vec<LeverageBracketQueryResp> = serde_json::from_str(&data).map_err(|e| AppError::new(-200, format!("{:?}", e).as_str()))?;

        let mut maint_margin_ratio = 0.0;
        if leverage_brackets.len() > 0 {
            for bracket in leverage_brackets[0].brackets.iter() {
                if bracket.initial_leverge <= config.leverage as usize {
                    maint_margin_ratio = bracket.maint_margin_ratio;
                    break;
                }
            }
        }
        let mut symbol_info = SymbolInfo {
            symbol: symbol.to_string(),
            leverage: config.leverage,
            margin_type: config.margin_type,
            dual_position_side: self.config.dual_position_side,
            multi_assets_margin: self.config.multi_assets_margin,
            maint_margin_ratio: maint_margin_ratio,
            quantity_precision: 0,
            price_precision: 0,
            quote_precision: 0,
        };

        if let Some(exchange_info) = self.exchange_info.as_ref() {
            for symbol_config in exchange_info.symbols.iter() {
                if symbol_config.symbol == symbol.to_string() {
                    symbol_info.quantity_precision = symbol_config.quantity_precision;
                    symbol_info.price_precision = symbol_config.price_precision;
                    symbol_info.quote_precision = symbol_config.quote_precision;
                    break;
                }
            }
        }
        Ok(symbol_info)
    }

    fn close(&self) {
        self.wss_stream.close();
    }
}