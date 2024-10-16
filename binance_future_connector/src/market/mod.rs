//! Market Data

pub mod agg_trades;
pub mod book_ticker;
pub mod depth;
pub mod exchange_info;
pub mod historical_trades;
pub mod klines;
pub mod ping;
pub mod ticker_price;
pub mod ticker_twenty_four_hr;
pub mod time;
pub mod trades;
pub mod contract_klines;
pub mod index_price_klines;
pub mod premium_index_klines;
pub mod premium_index;
pub mod funding_rate;
pub mod funding_info;
pub mod open_interest;

use agg_trades::AggTrades;
use book_ticker::BookTicker;
use contract_klines::{ContractKlines, ContractType};
use depth::Depth;
use exchange_info::ExchangeInfo;
use funding_info::FundingInfo;
use funding_rate::FundingRate;
use historical_trades::HistoricalTrades;
use index_price_klines::IndexPriceKlines;
use klines::{KlineInterval, Klines};
use open_interest::OpenInterest;
use ping::Ping;
use premium_index::PremiumIndex;
use premium_index_klines::PremiumIndexKlines;
use ticker_price::TickerPrice;
use ticker_twenty_four_hr::Ticker24hr;
use time::Time;
use trades::Trades;

pub fn ping() -> Ping {
    Ping::new()
}

pub fn time() -> Time {
    Time::new()
}

pub fn exchange_info() -> ExchangeInfo {
    ExchangeInfo::new()
}

pub fn depth(symbol: &str) -> Depth {
    Depth::new(symbol)
}

pub fn trades(symbol: &str) -> Trades {
    Trades::new(symbol)
}

pub fn historical_trades(symbol: &str) -> HistoricalTrades {
    HistoricalTrades::new(symbol)
}

pub fn agg_trades(symbol: &str) -> AggTrades {
    AggTrades::new(symbol)
}

pub fn klines(symbol: &str, interval: KlineInterval) -> Klines {
    Klines::new(symbol, interval)
}

pub fn contract_klines(pair: &str, contract_type: ContractType, interval: KlineInterval) -> ContractKlines {
    ContractKlines::new(pair, contract_type, interval)
}

pub fn index_price_klines(pair: &str, interval: KlineInterval) -> IndexPriceKlines {
    IndexPriceKlines::new(pair, interval)
}

pub fn premium_index_klines(pair: &str, interval: KlineInterval) -> PremiumIndexKlines {
    PremiumIndexKlines::new(pair, interval)
}

pub fn premium_index(symbol: &str) -> PremiumIndex {
    PremiumIndex::new(symbol)
}

pub fn funding_rate(symbol: &str) -> FundingRate {
    FundingRate::new(symbol)
}

pub fn funding_info() -> FundingInfo {
    FundingInfo::new()
}

pub fn ticker_twenty_four_hr() -> Ticker24hr {
    Ticker24hr::new()
}

pub fn ticker_price() -> TickerPrice {
    TickerPrice::new()
}

pub fn book_ticker() -> BookTicker {
    BookTicker::new()
}

pub fn open_interest(symbol: &str) -> OpenInterest {
    OpenInterest::new(symbol)
}
