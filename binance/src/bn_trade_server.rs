use std::sync::{Arc, RwLock};
use common::{error::AppError, msmc::{EventTrait, Subscription}, thread::{Handler, InteractiveThread, Rx}};
use serde_json::Value;
use binance_future_connector::{
    account,
    http::Credentials, trade::{self as bn_trade, new_order::NewOrderRequest}, ureq::{BinanceHttpClient, Error, Response}, user_data_stream, wss_listen_key_keepalive::WssListeneKeyKeepalive
};
use trade::trade_server::TradeServer;

use crate::model::{self, AccountInfo, Asset, Position};

#[derive(Clone)]
pub struct Config {
    pub api_key: String, 
    pub api_secret: String,
}

#[derive(Clone, Debug)]
pub enum AccountEvent {
    AccountUpdate(model::AccountUpdateEvent),
    OrderTradeUpdate(model::OrderTradeUpdateEvent),
    TradeLite(model::TradeLiteEvent),
}

impl EventTrait for AccountEvent {}

pub struct WssStream {
    pub subscription: Subscription<AccountEvent>,
}

impl WssStream {
    pub fn new() -> Self {
        WssStream {
            subscription: Subscription::top(),
        }
    }

    pub fn subscribe(&mut self) -> Subscription<AccountEvent> {
        self.subscription.subscribe()
    }

    pub fn connect(self, credentials: Credentials) {
        let credentials2 = credentials.clone();
        let mut keepalive = WssListeneKeyKeepalive::new(binance_future_connector::config::WSS_API).new_listen_key( move || {
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
            let data = message.into_data();
            let string_data = String::from_utf8(data).map_err(|e| Box::new(e))?;
            let json_value: Value = serde_json::from_str(&string_data).unwrap();
            match json_value.get("e") {
                Some(event_type) => {
                    let event = event_type.as_str().unwrap();
                    match event {
                        "ACCOUNT_UPDATE" => {
                            let account_update_event: model::AccountUpdateEvent = serde_json::from_str(&string_data).map_err(|e| Box::new(e))?;
                            self.subscription.send(&Some(AccountEvent::AccountUpdate(account_update_event)));
                        },
                        "ORDER_TRADE_UPDATE" => {
                            let order_trade_update_event: model::OrderTradeUpdateEvent = serde_json::from_str(&string_data).map_err(|e| Box::new(e))?;
                            self.subscription.send(&Some(AccountEvent::OrderTradeUpdate(order_trade_update_event)));
                        },
                        "TRADE_LITE" => {
                            let trade_lite_event: model::TradeLiteEvent = serde_json::from_str(&string_data).map_err(|e| Box::new(e))?;
                            self.subscription.send(&Some(AccountEvent::TradeLite(trade_lite_event)));
                        },
                        _ => {},
                    }
                },
                None => {},
            }
            Ok(true)
        }, true);
    }
}
pub struct BnTradeServer {
    pub config: Config,
    pub credentials: Credentials,
    pub positions: Arc<RwLock<Vec<model::Position>>>,
    pub assets: Arc<RwLock<Vec<model::Asset>>>,
    pub handler: Option<Handler<()>>,
}

impl BnTradeServer {
    pub fn new(config: Config) -> Self {
        BnTradeServer {
            credentials: Credentials::from_hmac(config.api_key.clone(), config.api_secret.clone()),
            config,
            positions: Arc::new(RwLock::new(Vec::new())),
            assets: Arc::new(RwLock::new(Vec::new())),
            handler: None,
        }
    }
    
    fn monitor_account_positions(&mut self, wss_sub: &mut Subscription<AccountEvent>) {
        let sub = wss_sub.subscribe();
        let assets_ref = self.assets.clone();
        let positions_ref = self.positions.clone();

        let closure = move |_: Rx<String>| {
            sub.stream(&mut |event| {
                if let Some(e) = event {
                    match e {
                        AccountEvent::AccountUpdate(a) => {
                            let mut positions = positions_ref.write().unwrap();
                            for position_data in a.update_data.positions.iter() {
                                for position in positions.iter_mut() {
                                    if position_data.symbol == position.symbol && position_data.position_side == position.position_side {
                                        position.position_amt = position_data.position_amount.clone();
                                        break;
                                    }
                                }
                            }
                            let mut assets = assets_ref.write().unwrap();
                            for balance_data in a.update_data.balances.iter() {
                                for asset in assets.iter_mut() {
                                    if balance_data.asset == asset.asset_name {
                                        asset.wallet_balance = balance_data.wallet_balance.clone();
                                        asset.cross_wallet_balance = balance_data.cross_wallet_balance.clone();
                                        break;
                                    }
                                }
                            }
                        },
                        _ => {},
                    }
                }
                true
            });
        };
        self.handler = Some(InteractiveThread::spawn(closure));
    }

    fn init_account_positions(&self) -> Result<(), AppError> {
        let client = BinanceHttpClient::default().credentials(self.credentials.clone());
        let data = self.get_resp_result(client.send(account::account()))?;
        let account_info: AccountInfo = serde_json::from_str(&data).map_err(|e| AppError::new(-200, format!("{:?}", e).as_str()))?;
        *self.assets.write().unwrap() = account_info.assets;
        *self.positions.write().unwrap() = account_info.positions;
        Ok(())
    }

    fn get_resp_result(&self, ret: Result<Response, Box<Error>>) -> Result<String, AppError> {
        let body = ret.map_err(|e| AppError::new(-200, format!("{:?}", e).as_str()))?;
        let data = body.into_body_str().map_err(|e| AppError::new(-200, format!("{:?}", e).as_str()))?;
        Ok(data)
    }
}

impl TradeServer for BnTradeServer {
    type Event = AccountEvent;
    type OrderRequest = NewOrderRequest;
    type Position = Position;
    type Account = Asset;

    fn connect(&mut self) -> Result<Subscription<AccountEvent>, AppError> {
        let mut wss_stream = WssStream::new();
        self.init_account_positions()?;
        self.monitor_account_positions(&mut wss_stream.subscription);

        let sub = wss_stream.subscription.subscribe();
        let credentials = self.credentials.clone();
        let _ = InteractiveThread::spawn(move |_| {
            wss_stream.connect(credentials);
        });
        Ok(sub)
    }

    fn new_order(&mut self, request : NewOrderRequest) -> Result<(), AppError> {
        let client = BinanceHttpClient::default().credentials(self.credentials.clone());
        let _ = client.send(request).map_err(|e| AppError::new(-200, format!("{:?}", e).as_str()))?;
        Ok(())
    }

    fn cancel_order(&mut self, symbol: &str, order_id: &str) -> Result<(), AppError> {
        let client = BinanceHttpClient::default().credentials(self.credentials.clone());
        let requset = bn_trade::cancel_order(symbol).orig_client_order_id(order_id);
        let _ = client.send(requset).map_err(|e| AppError::new(-200, format!("{:?}", e).as_str()))?;
        Ok(())
    }

    fn get_positions(&self, symbol: &str) -> Vec<Position> {
        let positions = self.positions.read().unwrap();
        let mut ret = vec![];
        for position in positions.iter() {
            if position.symbol == symbol {
                ret.push(position.clone());
            }
        }
        ret
    }

    fn get_account(&self, account_id: &str) -> Option<Asset>{
        let assets = self.assets.read().unwrap();
        let mut ret = None;
        for asset in assets.iter() {
            if account_id == asset.asset_name {
                ret = Some(asset.clone());
                break;
            }
        }
        ret
    }

    fn close(self) {
        if let Some(h) = self.handler {
            let _ = h.sender.send("QUIT".to_string());
        }
    }
}