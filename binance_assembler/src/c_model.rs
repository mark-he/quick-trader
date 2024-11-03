use binance_future_connector::trade::enums::{NewOrderResponseType, Side, TimeInForceType, OrderType, PositionSide, PriceMatchType};
use rust_decimal::Decimal;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize,)]
pub struct NewOrderRequest2 {
    pub symbol: String,
    pub side: Side,
    pub position_side: Option<PositionSide>,
    pub type_: OrderType,
    pub reduce_only: Option<String>,
    pub quantity: Option<Decimal>,
    pub price: Option<Decimal>,
    pub new_client_order_id: Option<String>,
    pub stop_price: Option<Decimal>,
    pub close_position: Option<String>,
    pub activation_price: Option<Decimal>,
    pub callback_rate: Option<Decimal>,
    pub time_in_force: Option<TimeInForceType>,
    pub working_type: Option<String>,
    pub price_protect: Option<String>,
    pub new_order_resp_type: Option<NewOrderResponseType>,
    pub price_match: Option<PriceMatchType>,
    pub self_trade_prevention_mode: Option<String>,
    pub good_till_date: Option<i64>,
    pub recv_window: Option<i64>,
}