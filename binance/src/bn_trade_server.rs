use std::{str::FromStr, sync::{Arc, Mutex, RwLock}};
use chrono::Local;
use common::{error::AppError, msmc::{EventTrait, Subscription}, thread::{Handler, InteractiveThread, Rx}};
use serde::{Serialize, Deserialize};
use serde_json::Value;
use binance_future_connector::{
    account,
    http::{error::ClientError, Credentials}, trade::{self as bn_trade, enums::{MarginAssetMode, MarginType}, new_order::NewOrderRequest}, ureq::{BinanceHttpClient, Error, Response}, user_data_stream, wss_listen_key_keepalive::WssListeneKeyKeepalive
};
use trade::trade_server::{SymbolRoute, TradeServer};

use crate::model::{self, Account, Asset, Position};

#[derive(Debug, Clone, Serialize, Deserialize,)]
pub struct Config {
    pub api_key: String, 
    pub api_secret: String,
    pub multi_assets_margin: String,
}

#[derive(Debug, Clone, Serialize, Deserialize,)]
pub struct SymbolConfig {
    pub margin_type: MarginType, 
    pub leverage: i32,
}

impl SymbolConfig {
    pub fn new() -> Self {
        SymbolConfig {
            margin_type: MarginType::Isolated,
            leverage: 5,
        }
    }
}

#[derive(Clone, Debug)]
pub enum AccountEvent {
    AccountUpdate(model::AccountData),
    OrderTradeUpdate(model::OrderData),
    TradeLite(model::TradeLiteEvent),
}

impl EventTrait for AccountEvent {}

impl SymbolRoute for AccountEvent {
    fn get_symbol(&self) -> String {
        match self {
            AccountEvent::OrderTradeUpdate(event) => {
                event.symbol.to_string()
            },
            AccountEvent::TradeLite(event) => {
                event.symbol.to_string()
            },
            _ => {
                "".to_string()
            }
        }
    }
}
pub struct WssStream {
    subscription: Arc<Mutex<Subscription<AccountEvent>>>,
    handler: Option<Handler<()>>,
}

impl WssStream {
    pub fn new() -> Self {
        WssStream {
            subscription: Arc::new(Mutex::new(Subscription::top())),
            handler : None,
        }
    }

    pub fn subscribe(&mut self) -> Subscription<AccountEvent> {
        self.subscription.lock().unwrap().subscribe()
    }

    pub fn connect(&mut self, credentials: Credentials) {
        let subscription_ref = self.subscription.clone();
        let closure = move |rx: Rx<String>| {
            let subscription = subscription_ref.lock().unwrap();
            let credentials2 = credentials.clone();
            let mut keepalive = WssListeneKeyKeepalive::new(&binance_future_connector::config::wss_api()).new_listen_key( move || {
                let client = BinanceHttpClient::default().credentials(credentials.clone());
                let request = user_data_stream::new_listen_key();
                let ret = client.send(request);

                match ret {
                    Ok(resp) => {
                        let data =  resp.into_body_str();
                        if let Ok(string_data) = data {
                            let json_value: Value = serde_json::from_str(&string_data).unwrap();
                            if let Some(key) = json_value.get("listenKey") {
                                return Some(key.as_str().unwrap().to_string());
                            }
                        }
                    },
                    _ => {
                    }
                }
                None
            }, 24 * 3600).renew_listen_key( move |listen_key| {
                let client = BinanceHttpClient::default().credentials(credentials2.clone());
                let request = user_data_stream::renew_listen_key(listen_key);
                let _ = client.send(request);
            }, 3600);

            let _ = keepalive.stream(|message| {
                let cmd = rx.try_recv();
                if cmd.is_ok() {
                    if cmd.unwrap() == "QUIT" {
                        return Ok(false);
                    }
                }
                let data = message.into_data();
                let string_data = String::from_utf8(data).map_err(|e| Box::new(e))?;
                let json_value: Value = serde_json::from_str(&string_data).unwrap();

                match json_value.get("e") {
                    Some(event_type) => {
                        let event = event_type.as_str().unwrap();
                        match event {
                            "ACCOUNT_UPDATE" => {
                                let account_update_event: model::AccountUpdateEvent = serde_json::from_str(&string_data).map_err(|e| Box::new(e))?;
                                subscription.send(&Some(AccountEvent::AccountUpdate(account_update_event.update_data)));
                            },
                            "ORDER_TRADE_UPDATE" => {
                                let order_trade_update_event= serde_json::from_str::<model::OrderTradeUpdateEvent>(&string_data).map_err(|e| Box::new(e))?;
                                subscription.send(&Some(AccountEvent::OrderTradeUpdate(order_trade_update_event.order)));
                            },
                            "TRADE_LITE" => {
                                let trade_lite_event: model::TradeLiteEvent = serde_json::from_str(&string_data).map_err(|e| Box::new(e))?;
                                subscription.send(&Some(AccountEvent::TradeLite(trade_lite_event)));
                            },
                            _ => {},
                        }
                    },
                    None => {},
                }
                Ok(true)
            }, true);
        };

        let handler = InteractiveThread::spawn(closure);
        self.handler = Some(handler);
    }

    fn close(self) {
        if let Some(h) = self.handler {
            let _ = h.sender.send("QUIT".to_string());
        }
    }
}
pub struct BnTradeServer {
    pub config: Config,
    pub credentials: Credentials,
    pub wss_stream: WssStream,
    pub positions: Arc<RwLock<Vec<model::Position>>>,
    pub assets: Arc<RwLock<Vec<model::Asset>>>,
    pub handler: Option<Handler<()>>,
}

impl BnTradeServer {
    pub fn new(config: Config) -> Self {
        BnTradeServer {
            credentials: Credentials::from_hmac(config.api_key.clone(), config.api_secret.clone()),
            config,
            wss_stream: WssStream::new(),
            positions: Arc::new(RwLock::new(Vec::new())),
            assets: Arc::new(RwLock::new(Vec::new())),
            handler: None,
        }
    }
    
    fn monitor_account_positions(&mut self, sub: Subscription<AccountEvent>) {
        let assets_ref = self.assets.clone();
        let positions_ref = self.positions.clone();

        let closure = move |_: Rx<String>| {
            let _ = sub.stream(&mut |event| {
                if let Some(e) = event {
                    match e {
                        AccountEvent::AccountUpdate(a) => {
                            let mut positions = positions_ref.write().unwrap();
                            for position_data in a.positions.iter() {
                                let mut found = false;
                                for position in positions.iter_mut() {
                                    if position_data.symbol == position.symbol && position_data.position_side == position.position_side {
                                        position.position_amt = position_data.position_amount.clone();
                                        position.entry_price = position_data.entry_price;
                                        position.unrealized_profit = position_data.unrealized_pnl;
                                        position.update_time = Local::now().timestamp_millis() as u64;
                                        found = true;
                                        break;
                                    }
                                }
                                if !found {
                                    positions.push(Position {
                                        symbol: position_data.symbol.clone(),
                                        position_side: position_data.position_side.clone(),
                                        position_amt: position_data.position_amount,
                                        unrealized_profit: position_data.unrealized_pnl,
                                        entry_price: position_data.entry_price,
                                        update_time: Local::now().timestamp_millis() as u64,
                                        ..Default::default()
                                    });
                                }
                            }

                            let mut assets = assets_ref.write().unwrap();
                            for balance_data in a.balances.iter() {
                                let mut found = false;
                                for asset in assets.iter_mut() {
                                    if balance_data.asset == asset.asset {
                                        asset.wallet_balance = balance_data.wallet_balance.clone();
                                        asset.cross_wallet_balance = balance_data.cross_wallet_balance.clone();
                                        found = true;
                                        break;
                                    }
                                }
                                if !found {
                                    assets.push(Asset {
                                        asset: balance_data.asset.clone(),
                                        wallet_balance: balance_data.wallet_balance.clone(),
                                        cross_wallet_balance: balance_data.cross_wallet_balance,
                                        update_time: Local::now().timestamp_millis() as u64,
                                        ..Default::default()
                                    });
                                }
                            }
                        },
                        _ => {},
                    }
                }
                Ok(true)
            }, true);
        };
        self.handler = Some(InteractiveThread::spawn(closure));
    }

    fn init_account(&self) -> Result<(), AppError> {
        let client = BinanceHttpClient::default().credentials(self.credentials.clone());
        let margin_asset_mode = MarginAssetMode::from_str(&self.config.multi_assets_margin).map_err(|e| AppError::new(-200, &e))?;
        let _ = Self::get_resp_result(client.send(bn_trade::multi_assets_margin(margin_asset_mode)), vec![-4171])?;
        Ok(())
    }

    fn init_account_positions(&self) -> Result<(), AppError> {
        let client = BinanceHttpClient::default().credentials(self.credentials.clone());
        let data = Self::get_resp_result(client.send(account::account()), vec![])?;
        let account_info: Account = serde_json::from_str(&data).map_err(|e| AppError::new(-200, format!("{:?}", e).as_str()))?;
        *self.assets.write().unwrap() = account_info.assets;
        *self.positions.write().unwrap() = account_info.positions;
        Ok(())
    }

    fn get_resp_result(ret: Result<Response, Box<Error>>, skipped_code: Vec<i16>) -> Result<String, AppError> {
        let err;
        match ret {
            Ok(resp) => {
                let ret2 = resp.into_body_str();
                match ret2 {
                    Ok(data) => {
                        return Ok(data);
                    },
                    Err(e) => {
                        err = *e;
                    },
                }
            },
            Err(e) => {
                err = *e;
            },
        }

        match err {
            Error::Client(ClientError::Structured(http)) => {
                if skipped_code.contains(&http.data.code) {
                    Ok("".to_string())
                } else {
                    Err(AppError::new(-200, format!("{:?}", &http.data.message).as_str()))
                }
            },
            _ => {
                Err(AppError::new(-200, format!("{:?}", err).as_str()))
            }
        }
    }
}

impl TradeServer for BnTradeServer {
    type Event = AccountEvent;
    type OrderRequest = NewOrderRequest;
    type Position = Position;
    type Account = Asset;
    type SymbolConfig = SymbolConfig;

    fn connect(&mut self) -> Result<Subscription<AccountEvent>, AppError> {
        self.init_account()?;
        self.init_account_positions()?;

        let sub = self.wss_stream.subscribe();
        self.monitor_account_positions(sub);

        let sub = self.wss_stream.subscribe();
        let credentials = self.credentials.clone();
        self.wss_stream.connect(credentials);
        Ok(sub)
    }

    fn new_order(&mut self, request : NewOrderRequest) -> Result<(), AppError> {
        let client = BinanceHttpClient::default().credentials(self.credentials.clone());
        let _ = Self::get_resp_result(client.send(request), vec![])?;
        Ok(())
    }

    fn cancel_order(&mut self, symbol: &str, order_id: &str) -> Result<(), AppError> {
        let client = BinanceHttpClient::default().credentials(self.credentials.clone());
        let requset = bn_trade::cancel_order(symbol).orig_client_order_id(order_id);
        let _ = Self::get_resp_result(client.send(requset), vec![])?;
        Ok(())
    }

    fn cancel_orders(&mut self, symbol: &str) -> Result<(), AppError> {
        let client = BinanceHttpClient::default().credentials(self.credentials.clone());
        let requset = bn_trade::cancel_open_orders(symbol);
        let _ = Self::get_resp_result(client.send(requset), vec![])?;
        Ok(())
    }

    fn get_positions(&self, symbol: &str) -> Vec<Position> {
        let positions = self.positions.read().unwrap();
        let mut ret = vec![];
        for position in positions.iter() {
            if position.symbol == symbol && position.position_amt != 0.0 {
                ret.push(position.clone());
            }
        }
        ret
    }

    fn get_account(&self, account_id: &str) -> Option<Asset>{
        let assets = self.assets.read().unwrap();
        let mut ret = None;
        for asset in assets.iter() {
            if account_id == asset.asset && asset.wallet_balance > 0.0 {
                ret = Some(asset.clone());
                break;
            }
        }
        ret
    }
    
    fn init_symbol(&self, symbol: &str, config: Self::SymbolConfig) -> Result<(), AppError> {
        let client = BinanceHttpClient::default().credentials(self.credentials.clone());

        let request = bn_trade::margin_type(symbol, config.margin_type);
        let _ = Self::get_resp_result(client.send(request), vec![-4046])?;

        let request = bn_trade::leverage(symbol, config.leverage);
        let _ = Self::get_resp_result(client.send(request), vec![])?;
        Ok(())
    }

    fn close(self) {
        if let Some(h) = self.handler {
            let _ = h.sender.send("QUIT".to_string());
        }
        self.wss_stream.close();
    }
}