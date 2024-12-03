use account::{AccountBalanceQueryRequest, AccountType};
use position::PositionQueryRequest;

use crate::enums::Category;

pub mod position;
pub mod account;

pub fn account(account_type: AccountType) -> AccountBalanceQueryRequest {
    AccountBalanceQueryRequest::new(account_type)
}

pub fn position(category: Category) -> PositionQueryRequest {
    PositionQueryRequest::new(category)
}
