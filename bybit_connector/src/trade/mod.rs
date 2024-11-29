use enums::{Category, OrderType, Side};
use new_order::NewOrderRequest;

pub mod new_order;
pub mod enums;

pub fn new_order(category: Category, symbol: &str, side: Side, order_type: OrderType, qty: &str) -> NewOrderRequest {
    NewOrderRequest::new(category, symbol, side, order_type, qty)
}
