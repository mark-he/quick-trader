use crate::http::{request::Request, Method};
use crate::trade::enums::{Side, TimeInForceType, OrderType, TriggerBy};
use rust_decimal::Decimal;
use serde::{Serialize, Deserialize};
use serde_json::Result;

use super::enums::Category;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewOrderRequest {
    pub category: Category,
    pub symbol: String,
    pub is_leverage: Option<i32>,
    pub side: Side,
    pub order_type: OrderType,
    pub qty: String,
    pub market_unit: Option<String>,
    pub price: Option<Decimal>,
    pub trigger_direction: Option<i32>,
    pub order_filter: Option<String>,
    pub trigger_price: Option<Decimal>,
    pub trigger_by: Option<TriggerBy>,
    pub order_iv: Option<Decimal>,
    pub time_in_force: Option<TimeInForceType>,
    pub position_idx: Option<i32>,
    pub order_link_id: Option<String>,
    pub take_profit: Option<Decimal>,
    pub stop_loss: Option<Decimal>,
    pub tp_trigger_by: Option<TriggerBy>,
    pub sl_trigger_by: Option<TriggerBy>,
    pub reduce_only: Option<bool>,
    pub close_on_trigger: Option<bool>,
    pub smp_type: Option<String>,
    pub mmp: Option<bool>,
    pub tpsl_mode: Option<String>,
    pub tp_limit_price: Option<Decimal>,
    pub sl_limit_price: Option<Decimal>,
    pub tp_order_type: Option<OrderType>,
    pub sl_order_type: Option<OrderType>,
}

impl NewOrderRequest {
    pub fn new(category: Category, symbol: &str, side: Side, order_type: OrderType, qty: &str) -> Self {
        Self {
            category: category.to_owned(),
            symbol: symbol.to_owned(),
            is_leverage: None,
            side,
            order_type,
            qty: qty.to_owned(),
            market_unit: None,
            price: None,
            trigger_direction: None,
            order_filter: None,
            trigger_price: None,
            trigger_by: None,
            order_iv: None,
            time_in_force: None,
            position_idx: None,
            order_link_id: None,
            take_profit: None,
            stop_loss: None,
            tp_trigger_by: None,
            sl_trigger_by: None,
            reduce_only: None,
            close_on_trigger: None,
            smp_type: None,
            mmp: None,
            tpsl_mode: None,
            tp_limit_price: None,
            sl_limit_price: None,
            tp_order_type: None,
            sl_order_type: None,
        }
    }

    pub fn is_leverage(mut self, is_leverage: i32) -> Self {
        self.is_leverage = Some(is_leverage);
        self
    }

    pub fn market_unit(mut self, market_unit: &str) -> Self {
        self.market_unit = Some(market_unit.to_owned());
        self
    }

    pub fn price(mut self, price: Decimal) -> Self {
        self.price = Some(price);
        self
    }

    pub fn trigger_direction(mut self, trigger_direction: i32) -> Self {
        self.trigger_direction = Some(trigger_direction);
        self
    }

    pub fn order_filter(mut self, order_filter: &str) -> Self {
        self.order_filter = Some(order_filter.to_owned());
        self
    }

    pub fn trigger_price(mut self, trigger_price: Decimal) -> Self {
        self.trigger_price = Some(trigger_price);
        self
    }

    pub fn trigger_by(mut self, trigger_by: TriggerBy) -> Self {
        self.trigger_by = Some(trigger_by.to_owned());
        self
    }

    pub fn order_iv(mut self, order_iv: Decimal) -> Self {
        self.order_iv = Some(order_iv);
        self
    }

    pub fn time_in_force(mut self, time_in_force: TimeInForceType) -> Self {
        self.time_in_force = Some(time_in_force);
        self
    }

    pub fn position_idx(mut self, position_idx: i32) -> Self {
        self.position_idx = Some(position_idx);
        self
    }

    pub fn order_link_id(mut self, order_link_id: &str) -> Self {
        self.order_link_id = Some(order_link_id.to_owned());
        self
    }

    pub fn take_profit(mut self, take_profit: Decimal) -> Self {
        self.take_profit = Some(take_profit);
        self
    }

    pub fn stop_loss(mut self, stop_loss: Decimal) -> Self {
        self.stop_loss = Some(stop_loss);
        self
    }

    pub fn tp_trigger_by(mut self, tp_trigger_by: TriggerBy) -> Self {
        self.tp_trigger_by = Some(tp_trigger_by.to_owned());
        self
    }

    pub fn sl_trigger_by(mut self, sl_trigger_by: TriggerBy) -> Self {
        self.sl_trigger_by = Some(sl_trigger_by.to_owned());
        self
    }

    pub fn reduce_only(mut self, reduce_only: bool) -> Self {
        self.reduce_only = Some(reduce_only);
        self
    }

    pub fn close_on_trigger(mut self, close_on_trigger: bool) -> Self {
        self.close_on_trigger = Some(close_on_trigger);
        self
    }

    pub fn smp_type(mut self, smp_type: &str) -> Self {
        self.smp_type = Some(smp_type.to_owned());
        self
    }

    pub fn mmp(mut self, mmp: bool) -> Self {
        self.mmp = Some(mmp);
        self
    }

    pub fn tpsl_mode(mut self, tpsl_mode: &str) -> Self {
        self.tpsl_mode = Some(tpsl_mode.to_owned());
        self
    }

    pub fn tp_limit_price(mut self, tp_limit_price: Decimal) -> Self {
        self.tp_limit_price = Some(tp_limit_price);
        self
    }

    pub fn sl_limit_price(mut self, sl_limit_price: Decimal) -> Self {
        self.sl_limit_price = Some(sl_limit_price);
        self
    }

    pub fn tp_order_type(mut self, tp_order_type: OrderType) -> Self {
        self.tp_order_type = Some(tp_order_type);
        self
    }

    pub fn sl_order_type(mut self, sl_order_type: OrderType) -> Self {
        self.sl_order_type = Some(sl_order_type);
        self
    }

    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string(&self)
    }
}

impl From<NewOrderRequest> for Request {
    fn from(request: NewOrderRequest) -> Request {
        Request {
            path: "/v5/order/create".to_owned(),
            method: Method::Post,
            params: vec![],
            credentials: None,
            sign: true,
            body: request.to_json().unwrap(),
            recv_window: 5000
        }
    }
}