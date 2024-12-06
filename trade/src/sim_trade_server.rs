use std::sync::{Arc, Mutex, RwLock};
use common::{error::AppError, msmc::Subscription};
use rust_decimal::{prelude::FromPrimitive, Decimal};
use serde::{Deserialize, Serialize};
use crate::trade_server::{Order, Position, TradeEvent, TradeServer, Wallet};


#[derive(Debug, Clone, Serialize, Deserialize,)]
pub struct SimTradeConfig {
    pub order_completed_status: String,
    pub asset: String,
    pub balance: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize,)]
pub struct NewOrderRequest {
    pub symbol: String,
    pub side: String,
    pub position_side: String,
    pub order_type: String,
    pub reduce_only: bool,
    pub quantity: Option<Decimal>,
    pub price: Option<Decimal>,
    pub new_client_order_id: Option<String>,
}

pub struct SimTradeServer {
    pub config: SimTradeConfig,
    pub positions: Arc<RwLock<Vec<Position>>>,
    pub assets: Arc<RwLock<Vec<Wallet>>>,
    pub subscription: Arc<Mutex<Subscription<TradeEvent>>>,
}

impl SimTradeServer {
    pub fn new(config: SimTradeConfig) -> Self {
        SimTradeServer {
            config,
            positions: Arc::new(RwLock::new(Vec::new())),
            assets: Arc::new(RwLock::new(Vec::new())),
            subscription: Arc::new(Mutex::new(Subscription::top())),
        }
    }
}

impl TradeServer for SimTradeServer {
    type OrderRequest = NewOrderRequest;
    type CancelOrderRequest = String;
    type SymbolConfig = String;
    type SymbolInfo = String;
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
        if request.order_type.to_string().to_uppercase() == "MARKET" {
            return Err(AppError::new(-200, "Sim Trade Server does not support MARKET order type"));
        }

        let mut positions = self.positions.write().unwrap();
        let mut found: Option<Position> = None;

        for p in positions.iter_mut() {
            if p.symbol == request.symbol && p.position_side == request.position_side {
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
            let quantity = request.quantity.unwrap().to_string().parse::<f64>().unwrap();
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
        } else {
            let p = found.as_ref().unwrap();
            if p.amount == 0.0 {
                let idx = positions.iter().position(|x| x == p);
                positions.remove(idx.unwrap());
            }
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
            order_type: request.order_type.clone(),
            price,
            total: quantity,
            traded: quantity,
            status: self.config.order_completed_status.clone(),
            ..Default::default()
        };
        sub.send(&TradeEvent::OrderUpdate(order_data));
        Ok(())
    }

    fn cancel_order(&mut self, _symbol: String, _request: String) -> Result<(), AppError> {
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
    
    fn init_symbol(&self, _symbol: String, _config: Self::SymbolConfig) -> Result<Self::SymbolInfo, AppError> {
        Ok("".to_string())
    }

    fn close(&self) {
    }
}

