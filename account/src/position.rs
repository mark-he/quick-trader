use rust_decimal::Decimal;

pub struct Position {
    pub symbol: String,
    pub side: String,
    pub amount: Decimal,
    pub price: Decimal,
}