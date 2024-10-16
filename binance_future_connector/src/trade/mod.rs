//! Account/Trade
//!
//! [API Documentation]()
pub mod account;
pub mod all_orders;
pub mod cancel_an_existing_order_and_send_a_new_order;
pub mod cancel_oco_order;
pub mod cancel_open_orders;
pub mod cancel_order;
pub mod get_oco_order;
pub mod get_oco_orders;
pub mod get_open_oco_orders;
pub mod get_order;
pub mod my_trades;
pub mod new_oco_order;
pub mod new_order;
pub mod new_order_test;
pub mod open_orders;
pub mod order;
pub mod order_limit_usage;
pub mod new_multi_order;
pub mod modify_order;
pub mod modify_multi_order;
pub mod order_amendment;
pub mod cancel_multi_order;
pub mod countdown_cancel_all;

use cancel_multi_order::CancelMultiOrder;
use countdown_cancel_all::CountdownCancelAll;
use modify_multi_order::ModifyMultiOrder;
use modify_order::ModifyOrder;
use new_multi_order::NewMultiOrder;
use order_amendment::OrderAmendment;
use rust_decimal::Decimal;
use account::Account;
use all_orders::AllOrders;
use cancel_an_existing_order_and_send_a_new_order::CancelAnExistingOrderAndSendANewOrder;
use cancel_oco_order::CancelOCOOrder;
use cancel_open_orders::CancelOpenOrders;
use cancel_order::CancelOrder;
use get_oco_order::GetOCOOrder;
use get_oco_orders::GetOCOOrders;
use get_open_oco_orders::GetOpenOCOOrders;
use get_order::GetOrder;
use my_trades::MyTrades;
use new_oco_order::NewOCOOrder;
use new_order::NewOrder;
use new_order_test::NewOrderTest;
use open_orders::OpenOrders;
use order::{CancelReplaceMode, OrderType, Side};
use order_limit_usage::OrderLimitUsage;

pub fn new_order_test(symbol: &str, side: Side, r#type: &str) -> NewOrderTest {
    NewOrderTest::new(symbol, side, r#type)
}

pub fn get_order(symbol: &str) -> GetOrder {
    GetOrder::new(symbol)
}

pub fn cancel_an_existing_order_and_send_a_new_order(
    symbol: &str,
    side: Side,
    r#type: &str,
    cancel_replace_mode: CancelReplaceMode,
) -> CancelAnExistingOrderAndSendANewOrder {
    CancelAnExistingOrderAndSendANewOrder::new(symbol, side, r#type, cancel_replace_mode)
}

pub fn new_order(symbol: &str, side: Side, type_: OrderType) -> NewOrder {
    NewOrder::new(symbol, side, type_)
}

pub fn modify_order(symbol: &str, side: Side, quantity: Decimal, price: Decimal) -> ModifyOrder {
    ModifyOrder::new(symbol, side, quantity, price)
}

pub fn new_multi_order() -> NewMultiOrder {
    NewMultiOrder::new()
}

pub fn modify_multi_order() -> ModifyMultiOrder {
    ModifyMultiOrder::new()
}

pub fn order_amendment(symbol: &str) -> OrderAmendment {
    OrderAmendment::new(symbol)
}

pub fn cancel_order(symbol: &str) -> CancelOrder {
    CancelOrder::new(symbol)
}

pub fn cancel_multi_order(symbol: &str) -> CancelMultiOrder {
    CancelMultiOrder::new(symbol)
}

pub fn open_orders() -> OpenOrders {
    OpenOrders::new()
}

pub fn cancel_open_orders(symbol: &str) -> CancelOpenOrders {
    CancelOpenOrders::new(symbol)
}

pub fn countdown_cancel_all(symbol: &str, countdown_time: i64) -> CountdownCancelAll {
    CountdownCancelAll::new(symbol, countdown_time)
}

pub fn all_orders(symbol: &str) -> AllOrders {
    AllOrders::new(symbol)
}

pub fn new_oco_order(
    symbol: &str,
    side: Side,
    quantity: Decimal,
    price: Decimal,
    stop_price: Decimal,
) -> NewOCOOrder {
    NewOCOOrder::new(symbol, side, quantity, price, stop_price)
}

pub fn get_oco_order() -> GetOCOOrder {
    GetOCOOrder::new()
}

pub fn cancel_oco_order(symbol: &str) -> CancelOCOOrder {
    CancelOCOOrder::new(symbol)
}

pub fn get_oco_orders() -> GetOCOOrders {
    GetOCOOrders::new()
}

pub fn get_open_oco_orders() -> GetOpenOCOOrders {
    GetOpenOCOOrders::new()
}

pub fn account() -> Account {
    Account::new()
}

pub fn my_trades(symbol: &str) -> MyTrades {
    MyTrades::new(symbol)
}

pub fn order_limit_usage() -> OrderLimitUsage {
    OrderLimitUsage::new()
}
