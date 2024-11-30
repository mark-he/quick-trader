use cancel_order::CancelOrderRequest;
use cancel_orders::CancelOrdersRequest;
use crate::enums::{Category, OrderType, Side};
use new_order::NewOrderRequest;
use set_leverage::SetLeverageRequest;
use set_margin_mode::SetMarginModeRequest;
use switch_mode::SwitchModeRequest;

pub mod new_order;
pub mod cancel_order;
pub mod cancel_orders;
pub mod set_margin_mode;
pub mod set_leverage;
pub mod switch_mode;

pub fn new_order(category: Category, symbol: &str, side: Side, order_type: OrderType, qty: &str) -> NewOrderRequest {
    NewOrderRequest::new(category, symbol, side, order_type, qty)
}

pub fn cancel_order(category: Category, symbol: &str) -> CancelOrderRequest {
    CancelOrderRequest::new(category, symbol)
}

pub fn cancel_orders(category: Category, symbol: &str) -> CancelOrdersRequest {
    CancelOrdersRequest::new(category, symbol)
}

pub fn set_leverage(category: Category, symbol: &str, buy_leverage: &str, sell_leverage: &str) -> SetLeverageRequest {
    SetLeverageRequest::new(category, symbol, buy_leverage, sell_leverage)
}

pub fn set_margin_mode(set_margin_mode: &str) -> SetMarginModeRequest {
    SetMarginModeRequest::new(set_margin_mode)
}

pub fn switch_mode(category: Category, mode: u32) -> SwitchModeRequest {
    SwitchModeRequest::new(category, mode)
}