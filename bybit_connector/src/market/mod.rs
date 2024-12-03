use klines::GetKlinesRequest;
use time::GetServerTimeRequest;

use crate::enums::{Category, KlineInterval};

pub mod klines;
pub mod time;

pub fn klines(category: Category, symbol: &str, interval: KlineInterval) -> GetKlinesRequest {
    GetKlinesRequest::new(category, symbol, interval)
}

pub fn time() -> GetServerTimeRequest {
    GetServerTimeRequest::new()
}