use std::collections::HashMap;
use std::f64::MAX;
use std::sync::{Arc, RwLock};
use std::thread;
use std::cmp::min;
use market::market_server::MarketData;
use trade::code;
use {common::msmc::Subscription, trade::trade_server::*};
use common::error::AppError;

type SharedAs = Arc<RwLock<AccountStorage>>;

static mut ACCOUNT_DB: Option<SharedAs> = None;
pub struct AccountStorage {
    accounts : HashMap<String, f64>,
    orders : Vec<Order>,
    history_orders: Vec<Order>,
    trades : Vec<Trade>,
    positions : Vec<Position>,
}

impl AccountStorage {
    pub fn init() {
        unsafe {
            ACCOUNT_DB = Some(Arc::new(RwLock::new(AccountStorage {
                accounts: HashMap::new(),
                orders : vec![],
                history_orders: vec![],
                trades : vec![],
                positions: vec![],
            })));
        }
    }

    pub fn get_instance() -> Arc<RwLock<AccountStorage>> {
        unsafe {
            ACCOUNT_DB.as_ref().unwrap().clone()
        }
    }

    fn get_positions(&self, unit_id: &str, symbol: &str) -> Vec<Position> {
        let mut ret = Vec::<Position>::new();
        for item in &self.positions {
            if item.invest_unit_id == unit_id && item.symbol == symbol {
                ret.push(item.clone());
            }
        }
        ret
    }

    fn get_account(&self, unit_id: &str) -> Account {
        let opt = self.accounts.get(unit_id);
        let mut interest = 0.0;
        if opt.is_some() {
            interest = *opt.unwrap();
        }
        Account {
            account_id: unit_id.to_string(),
            interest,
            available: MAX,
            balance: MAX,
        }
    }

    pub fn create_order(&mut self, order : &Order) {
        self.orders.push(order.clone());
    }

    pub fn find_order(&mut self, sys_id: &str) -> Option<usize> {
        let mut index = 0;
        while index < self.orders.len() {
            let temp = &self.orders[index];
            if temp.sys_id == sys_id {
                return Some(index)
            } else {
                index += 1;
            }
        }
        None
    }

    pub fn cancel_order(&mut self, sys_id: &str) -> Option<Order> {
        let ret = self.find_order(sys_id);
        match ret {
            Some(index) => {
                let mut order = self.orders.remove(index);
                order.status = code::ORDER_STATUS_CANCELLED.to_string();
                order.status_msg = String::from("Cancelled");
                self.history_orders.push(order.clone());
                Some(order)
            },
            None => {
                None
            },
        }
    }

    #[allow(unused_assignments)]
    pub fn trade_order(&mut self, sys_id: &str, price: f64, datetime: &str, trading_day: &str) -> Option<(Order, Trade)> {
        
        let ret = self.find_order(sys_id);
        if let Some(index) = ret {
            let mut traded_volume = 0;
            let orders = &self.orders;
            let order = orders.get(index).unwrap();
            
            let mut ret_order : Order = order.clone();

            if order.offset == code::OFFSET_OPEN.to_string() {
                let pos = Position {
                    symbol: order.symbol.clone(),
                    position: order.volume_total,
                    today_position: order.volume_total,
                    direction: order.direction.clone(),
                    cost: order.price,
                    cost_offset: 0.0,
                    trading_day: trading_day.to_string(),
                    invest_unit_id : ret_order.invest_unit_id.clone(),
                };
                self.positions.push(pos);
                traded_volume = order.volume_total;
            } else { //CLOSE
                let mut account_offset = 0.0;
                let mut trade_left = order.volume_total;

                let mut index = 0;
                while index < self.positions.len() {
                    if trade_left > 0 {
                        let temp = &mut self.positions[index];
                        if temp.symbol == order.symbol && temp.invest_unit_id == order.invest_unit_id {
                            if temp.direction != order.direction {
                                let trade_vol = min(min(trade_left, order.volume_total), temp.position);
                                trade_left -= trade_vol;

                                if temp.direction == code::DIRECTION_LONG {
                                    account_offset += (order.price - temp.cost) * 10 as f64 * trade_vol as f64;
                                } else {
                                    account_offset += (temp.cost - order.price) * 10 as f64 * trade_vol as f64;
                                }
                                if trade_vol == temp.position {
                                    self.positions.remove(index);
                                } else {
                                    temp.position -= trade_vol;
                                }
                            }
                        } else {
                            index += 1;
                        }
                    } else {
                        break;
                    }
                }
                traded_volume = order.volume_total - trade_left;
                let interest = self.accounts.entry(ret_order.invest_unit_id.clone()).or_insert(0.0);
                *interest += account_offset;
            }

            let trade = Trade {
                order_ref: order.order_ref.clone(),
                trade_id: 0.to_string(),
                sys_id: order.sys_id.clone(),
                direction: order.direction.clone(),
                offset: order.offset.clone(),
                price: price,
                volume: traded_volume,
                datetime: datetime.to_string(),
                symbol: order.symbol.to_string(),
            };
            self.trades.push(trade.clone());
            
            ret_order.volume_traded += traded_volume;
            ret_order.volume_total -= traded_volume;

            if ret_order.volume_total == 0 {
                ret_order.status = code::ORDER_STATUS_ALL_TRADED.to_string();
                ret_order.status_msg = String::from("All Traded");

                self.history_orders.push(order.clone());
                let orders = &mut self.orders;
                orders.remove(index);
            } else if ret_order.volume_traded > 0 {   
                ret_order.status = code::ORDER_STATUS_PART_TRADED_QUEUEING.to_string();
            }
            Some((ret_order, trade))
        } else {
            None
        }
    }
    
}

pub struct BacktestTradeServer {
    market_sub : Option<Arc<RwLock<Subscription<MarketData>>>>,
    trade_sub : Arc<RwLock<Subscription<(i32, TradeData)>>>,
    order_id_gen: u32,
}

impl BacktestTradeServer {
    pub fn new() -> Self {
        BacktestTradeServer {
            market_sub: None,
            trade_sub: Arc::new(RwLock::new(Subscription::<(i32, TradeData)>::top())),
            order_id_gen: 0,
        }
    }

    pub fn set_market_sub(&mut self, subscription: Subscription<MarketData>) {
        self.market_sub = Some(Arc::new(RwLock::new(subscription)));
    }
}

impl TradeServer for BacktestTradeServer {
    #[allow(unused_variables)]
    fn connect(&mut self, config: &TradeConfig) -> Result<Subscription<(i32, TradeData)>, AppError> {
        AccountStorage::init();

        let outer_sub = self.trade_sub.write().unwrap().subscribe_with_filter(|event|{
            match event {
                (_, TradeData::OnOrder(_)) | (_, TradeData::OnTrade(_)) => {
                    true
                },
                _ => {
                    false
                }
            }
        });
        let market_sub_ref = self.market_sub.as_ref().unwrap().clone();
        let top_sub = self.trade_sub.clone();
        
        thread::spawn(move || {
            let market_sub = market_sub_ref.read().unwrap();
            loop {
                let _ = market_sub.recv(&mut |event| {
                    match event {
                        Some(data) => {
                            let mut ret = None;
                            match data {
                                MarketData::Tick(t) => {
                                    let account_db = AccountStorage::get_instance();
                                    let account = account_db.read().unwrap();
                                    let orders = account.orders.clone();
                                    drop(account);
                                    for item in orders {
                                        if item.symbol == t.symbol {
                                            let mut account = account_db.write().unwrap();
                                            if item.direction == code::DIRECTION_LONG {
                                                if t.last_price < item.price {
                                                    ret = account.trade_order(&item.sys_id, t.last_price, &t.datetime, &t.trading_day);
                                                }
                                            } else {
                                                if t.last_price > item.price {
                                                    ret = account.trade_order(&item.sys_id, t.last_price, &t.datetime, &t.trading_day);
                                                }
                                            }
                                        }
                                    }
                                },
                                _ => {},
                            }

                            if let Some((order, trade)) = ret {
                                let sender = top_sub.read().unwrap();
                                sender.send(&Some((order.request_id.clone(), TradeData::OnOrder(order.clone()))));
                                sender.send(&Some((order.request_id.clone(), TradeData::OnTrade(trade.clone()))));
                            }
                        },
                        None => {},
                    }
                });
            }
        });
        Ok(outer_sub)
    }

    fn send_order(&mut self, order_insert : &OrderInsert, unit_id: &str, request_id : i32) {
        self.order_id_gen += 1;
        let order = Order { 
            order_ref: order_insert.order_ref.clone(), 
            direction: order_insert.direction.clone(), 
            offset: order_insert.offset.clone(), 
            price: order_insert.limit_price, 
            volume_total_original: order_insert.volume_total, 
            submit_status: code::ORDER_SUBMIT_ACCEPTED_SUBMITTED.to_string(), 
            sys_id: self.order_id_gen.to_string(), 
            order_type: order_insert.order_type.clone(),
            status: code::ORDER_STATUS_NO_TRADED_QUEUEING.to_string(), 
            volume_traded: 0, 
            volume_total: order_insert.volume_total, 
            status_msg: "已提交未成交".to_string(), 
            symbol: order_insert.symbol.clone(),
            request_id: request_id,
            invest_unit_id: unit_id.to_string(),
        };
        let account_db = AccountStorage::get_instance();
        let mut account = account_db.write().unwrap();
        account.create_order(&order);
        self.trade_sub.read().unwrap().send(&Some((request_id, TradeData::OnOrder(order.clone()))));

    }

    fn cancel_order(&mut self, action: &OrderAction, request_id : i32) {
        let account_db = AccountStorage::get_instance();
        let mut account = account_db.write().unwrap();
        let ret = account.cancel_order(&action.sys_id);
        
        match ret {
            Some(order) => {
                self.trade_sub.read().unwrap().send(&Some((request_id, TradeData::OnOrder(order))));
            },
            None => {
                self.trade_sub.read().unwrap().send(&Some((request_id, TradeData::Error(-200, "订单已撤销或已成交".to_string()))));
            },
        }
    }

    fn get_positions(&self, unit_id: &str, symbol: &str) -> Vec<Position> {
        let account_db = AccountStorage::get_instance();
        let account = account_db.read().unwrap();
        account.get_positions(unit_id, symbol)
    }

    fn get_account(&self, unit_id: &str) -> Account {
        let account_db = AccountStorage::get_instance();
        let account = account_db.read().unwrap();
        account.get_account(unit_id)
    }    
    
    fn session(&self) -> Option<TradeSession> {
        None
    }
}
