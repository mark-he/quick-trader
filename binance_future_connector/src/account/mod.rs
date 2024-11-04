pub mod account;
pub mod leverage_bracket;

use account::AccountRequest;
use leverage_bracket::LeverageBracketRequest;

pub fn account() -> AccountRequest {
    AccountRequest::new()
}

pub fn leverageBracket() -> LeverageBracketRequest {
    LeverageBracketRequest::new()
}