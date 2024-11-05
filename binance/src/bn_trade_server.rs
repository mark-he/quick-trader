use std::sync::{Arc, Mutex, RwLock};
use chrono::Local;
use common::{error::AppError, msmc::{EventTrait, Subscription}, thread::{Handler, InteractiveThread, Rx}};
use serde::{Serialize, Deserialize};
use serde_json::Value;
use binance_future_connector::{
    account, http::Credentials, market as bn_market, trade::{self as bn_trade, enums::{MarginAssetMode, MarginType, PositionMode}, new_order::NewOrderRequest}, ureq::BinanceHttpClient, user_data_stream, wss_listen_key_keepalive::WssListeneKeyKeepalive
};
use trade::trade_server::{SymbolRoute, TradeServer};

use crate::model::{self, Account, Asset, ExchangeInfo, LeverageBracket, Position};

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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SymbolInfo {
    pub symbol: String, 
    pub leverage: i32,
    pub margin_type: MarginType,
    pub dual_position_side: PositionMode,
    pub multi_assets_margin: MarginAssetMode,
    pub maint_margin_ratio: f64,
    pub quantity_precision: usize,
    pub price_precision: usize,
    pub quote_precision: usize,
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
                let string_data = model::get_resp_result(client.send(request), vec![])?;

                let json_value: Value = serde_json::from_str(&string_data).unwrap();
                if let Some(key) = json_value.get("listenKey") {
                    Ok(key.as_str().unwrap().to_string())
                } else {
                    Err(Box::new(AppError::new(-200, format!("{:?}", "listenKey not found").as_str())))
                }
            }, 24 * 3600).renew_listen_key( move |listen_key| {
                let client = BinanceHttpClient::default().credentials(credentials2.clone());
                let request = user_data_stream::renew_listen_key(listen_key);
                let _ = model::get_resp_result(client.send(request), vec![])?;
                Ok(())
            }, 3600);

            let _ = keepalive.stream(&mut |message| {
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
                    None => {
                        println!("Received {}", string_data);
                    },
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
    pub config: model::Config,
    pub credentials: Credentials,
    pub wss_stream: WssStream,
    pub positions: Arc<RwLock<Vec<model::Position>>>,
    pub assets: Arc<RwLock<Vec<model::Asset>>>,
    pub exchange_info: Option<ExchangeInfo>,
    pub handler: Option<Handler<()>>,
}

impl BnTradeServer {
    pub fn new(config: model::Config) -> Self {
        BnTradeServer {
            credentials: Credentials::from_hmac(config.api_key.clone(), config.api_secret.clone()),
            config,
            wss_stream: WssStream::new(),
            positions: Arc::new(RwLock::new(Vec::new())),
            assets: Arc::new(RwLock::new(Vec::new())),
            exchange_info: None,
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
        let _ = model::get_resp_result(client.send(bn_trade::multi_assets_margin(self.config.multi_assets_margin)), vec![-4171])?;
        let _ = model::get_resp_result(client.send(bn_trade::position_side(self.config.dual_position_side)), vec![-4059])?;
        Ok(())
    }

    fn init_exchange(&mut self) -> Result<(), AppError> {
        let client = BinanceHttpClient::default().credentials(self.credentials.clone());
        let data = model::get_resp_result(client.send(bn_market::exchange_info()), vec![])?;
        let exchange_info: ExchangeInfo = serde_json::from_str(&data).map_err(|e| AppError::new(-200, format!("{:?}", e).as_str()))?;
        self.exchange_info = Some(exchange_info);
        Ok(())
    }

    fn init_account_positions(&self) -> Result<(), AppError> {
        let client = BinanceHttpClient::default().credentials(self.credentials.clone());
        let data = model::get_resp_result(client.send(account::account()), vec![])?;
        let account_info: Account = serde_json::from_str(&data).map_err(|e| AppError::new(-200, format!("{:?}", e).as_str()))?;
        *self.assets.write().unwrap() = account_info.assets;
        *self.positions.write().unwrap() = account_info.positions;
        Ok(())
    }

}

impl TradeServer for BnTradeServer {
    type Event = AccountEvent;
    type OrderRequest = NewOrderRequest;
    type Position = Position;
    type Account = Asset;
    type SymbolConfig = SymbolConfig;
    type SymbolInfo = SymbolInfo;

    fn connect(&mut self) -> Result<Subscription<AccountEvent>, AppError> {
        self.init_exchange()?;
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
        println!("{:?}", request);
        let client = BinanceHttpClient::default().credentials(self.credentials.clone());
        let _ = model::get_resp_result(client.send(request), vec![])?;
        Ok(())
    }

    fn cancel_order(&mut self, symbol: &str, order_id: &str) -> Result<(), AppError> {
        let client = BinanceHttpClient::default().credentials(self.credentials.clone());
        let requset = bn_trade::cancel_order(symbol).orig_client_order_id(order_id);
        let _ = model::get_resp_result(client.send(requset), vec![])?;
        Ok(())
    }

    fn cancel_orders(&mut self, symbol: &str) -> Result<(), AppError> {
        let client = BinanceHttpClient::default().credentials(self.credentials.clone());
        let requset = bn_trade::cancel_open_orders(symbol);
        let _ = model::get_resp_result(client.send(requset), vec![])?;
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
    
    fn init_symbol(&self, symbol: &str, config: Self::SymbolConfig) -> Result<SymbolInfo, AppError> {
        let client = BinanceHttpClient::default().credentials(self.credentials.clone());

        let request = bn_trade::margin_type(symbol, config.margin_type);
        let _ = model::get_resp_result(client.send(request), vec![-4046])?;

        let request = bn_trade::leverage(symbol, config.leverage);
        let _ = model::get_resp_result(client.send(request), vec![])?;

        let request = account::leverage_bracket().symbol(symbol);
        let data = model::get_resp_result(client.send(request), vec![])?;

        let leverage_brackets: Vec<LeverageBracket> = serde_json::from_str(&data).map_err(|e| AppError::new(-200, format!("{:?}", e).as_str()))?;

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
                if symbol_config.symbol == symbol {
                    symbol_info.quantity_precision = symbol_config.quantity_precision;
                    symbol_info.price_precision = symbol_config.price_precision;
                    symbol_info.quote_precision = symbol_config.quote_precision;
                    break;
                }
            }
        }
        Ok(symbol_info)
    }

    fn close(self) {
        if let Some(h) = self.handler {
            let _ = h.sender.send("QUIT".to_string());
        }
        self.wss_stream.close();
    }
}