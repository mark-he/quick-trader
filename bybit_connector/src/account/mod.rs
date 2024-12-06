use account::{AccountBalanceQueryRequest, AccountType};
use account_info::AccountInfoRequest;
use position::PositionQueryRequest;

use crate::enums::Category;

pub mod position;
pub mod account;
pub mod account_info;

pub fn account(account_type: AccountType) -> AccountBalanceQueryRequest {
    AccountBalanceQueryRequest::new(account_type)
}

pub fn position(category: Category) -> PositionQueryRequest {
    PositionQueryRequest::new(category)
}

pub fn account_info() -> AccountInfoRequest {
    AccountInfoRequest::new()
}
