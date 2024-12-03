use std::sync::{Arc, Mutex, RwLock};
use common::{error::AppError, msmc::Subscription};
use binance_future_connector::trade::{enums::{MarginAssetMode, OrderType, PositionMode, PositionSide, Side}, new_order::NewOrderRequest};
use rust_decimal::{prelude::FromPrimitive, Decimal};
use trade::trade_server::{Order, Position, TradeEvent, TradeServer, Wallet};
use binance::{bn_trade_server::BnTradeServerTrait, model::*};

use crate::model::SimTradeConfig;

impl BnTradeServerTrait for BnSimTradeServer {}
pub struct BnSimTradeServer {
    pub config: SimTradeConfig,
    pub positions: Arc<RwLock<Vec<Position>>>,
    pub assets: Arc<RwLock<Vec<Wallet>>>,
    pub subscription: Arc<Mutex<Subscription<TradeEvent>>>,
}

impl BnSimTradeServer {
    pub fn new(config: SimTradeConfig) -> Self {
        BnSimTradeServer {
            config,
            positions: Arc::new(RwLock::new(Vec::new())),
            assets: Arc::new(RwLock::new(Vec::new())),
            subscription: Arc::new(Mutex::new(Subscription::top())),
        }
    }
}

impl TradeServer for BnSimTradeServer {
    type OrderRequest = NewOrderRequest;
    type CancelOrderRequest = CancelOrderRequest;
    type SymbolConfig = SymbolConfig;
    type SymbolInfo = SymbolInfo;
    type Symbol = String;
    
    fn init(&mut self) -> Result<(), AppError> {
        self.assets.write().unwrap().push(Wallet {
            asset: self.config.asset.clone(),
            balance: self.config.balance as f64,
            available_balance: self.config.balance as f64,
            ..Default::default()
        });
        Ok(())
    }

    fn start(&mut self) -> Result<Subscription<TradeEvent>, AppError> {
        let mut top = Subscription::<TradeEvent>::top();
        let sub = top.subscribe();
        self.subscription = Arc::new(Mutex::new(top));
        Ok(sub)
    }

    fn new_order(&mut self, _symbol: String, mut request : NewOrderRequest) -> Result<(), AppError> {
        if request.type_.to_string() == OrderType::Market.to_string() {
            return Err(AppError::new(-200, "Sim Trade Server does not support MARKET order type"));
        }

        let mut positions = self.positions.write().unwrap();
        //let mut assets = self.assets.write().unwrap();
        let mut found: Option<Position> = None;

        if request.position_side.is_none() {
            request.position_side = Some(PositionSide::Both);
        }

        for p in positions.iter_mut() {
            if p.symbol == request.symbol && p.position_side == request.position_side.unwrap().to_string() {
                let quantity: f64;
                if let Some(v) = request.quantity {
                    quantity = v.to_string().parse::<f64>().unwrap();
                } else {
                    quantity = p.amount;
                }
                request.quantity = Decimal::from_f64(quantity);
                if p.side != request.side.to_string() {
                    p.amount = p.amount - quantity;
                } else {
                    p.amount = p.amount + quantity;
                }
                found = Some(p.clone());
                break;
            }
        }

        if found.is_none() {
            let mut quantity = request.quantity.unwrap().to_string().parse::<f64>().unwrap();
            if request.side.to_string() == Side::Sell.to_string() {
                quantity = 0.0 - quantity;
            } 
            let p = Position {
                symbol: request.symbol.clone(),
                cost: request.price.unwrap().to_string().parse::<f64>().unwrap(),
                position_side: "BOTH".to_string(),
                side: request.side.to_string(),
                amount: quantity,
                ..Default::default()
            };
            positions.push(p.clone());
            found = Some(p);
        }

        let sub = self.subscription.lock().unwrap();
        let p = found.take().unwrap();
        sub.send(&TradeEvent::PositionUpdate(p));

        let quantity = request.quantity.unwrap().to_string().parse::<f64>().unwrap();
        let mut price = 0 as f64;
        if let Some(v) = request.price {
            price = v.to_string().parse::<f64>().unwrap();
        }
        let order_data = Order {
            symbol: request.symbol.clone(),
            client_order_id: request.new_client_order_id.unwrap().clone(),
            side: request.side.to_string(),
            order_type: request.type_.to_string(),
            price,
            total: quantity,
            traded: quantity,
            ..Default::default()
        };
        sub.send(&TradeEvent::OrderUpdate(order_data));
        Ok(())
    }

    fn cancel_order(&mut self, _symbol: String, _request: CancelOrderRequest) -> Result<(), AppError> {
        Ok(())
    }

    fn cancel_orders(&mut self, _symbol: String) -> Result<(), AppError> {
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
        let assets = self.assets.read().unwrap();
        let mut ret = None;
        for asset in assets.iter() {
            if account_id == asset.asset && asset.balance > 0.0 {
                ret = Some(asset.clone());
                break;
            }
        }
        Ok(ret)
    }
    
    fn init_symbol(&self, symbol: String, config: Self::SymbolConfig) -> Result<SymbolInfo, AppError> {
        let symbol_info = SymbolInfo {
            symbol: symbol.to_string(),
            leverage: config.leverage,
            margin_type: config.margin_type,
            dual_position_side: PositionMode::OneWayMode,
            multi_assets_margin: MarginAssetMode::SingleAsset,
            maint_margin_ratio: leverage_margin_ratio(config.leverage),
            quantity_precision: 8,
            price_precision: 8,
            quote_precision: 8,
        };
        Ok(symbol_info)
    }

    fn close(&self) {
    }
}

fn leverage_margin_ratio(leverage: i32) -> f64 {
    1.0 as f64 / leverage as f64 / 4.0 as f64
}

