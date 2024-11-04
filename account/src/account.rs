#[derive(Debug, Default)]
pub struct Account {
    pub balance : f64,
    pub available : f64,
    pub interest : f64,
    pub account_id : String,
}

impl Account {
    pub fn new() -> Self {
        Account {
            ..Default::default()
        }
    }
}

