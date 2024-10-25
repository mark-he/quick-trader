pub mod account;

use account::AccountRequest;

pub fn account() -> AccountRequest {
    AccountRequest::new()
}