use std::str::FromStr;
use strum::Display;

pub fn optional<T: FromStr>(value: &str) -> Result<Option<T>, <T as FromStr>::Err> {
    if value == "" {
        Ok(None)
    } else {
        let ret = T::from_str(value)?;
        Ok(Some(ret))
    }
}

#[derive(Copy, Clone, Display)]
pub enum PositionMarginType {
    #[strum(serialize = "1")]
    Add,
    #[strum(serialize = "2")]
    Reduce,
}

#[derive(Copy, Clone, Display)]
pub enum PositionSide {
    #[strum(serialize = "LONG")]
    Long,
    #[strum(serialize = "SHORT")]
    Short,
    #[strum(serialize = "BOTH")]
    Both,
}

impl FromStr for PositionSide {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "LONG" => Ok(PositionSide::Long),
            "SHORT" => Ok(PositionSide::Short),
            "BOTH" => Ok(PositionSide::Both),
            _ => Err("Invalid PositionSide".to_string()),
        }
    }
}

#[derive(Copy, Clone, Display)]
pub enum MarginAssetMode {
    #[strum(serialize = "true")]
    MultiAssets,
    #[strum(serialize = "false")]
    SingleAsset,
}

#[derive(Copy, Clone, Display)]
pub enum PositionMode {
    #[strum(serialize = "true")]
    HedgeMode,
    #[strum(serialize = "false")]
    OneWayMode,
}

#[derive(Copy, Clone, Display)]
pub enum MarginType {
    #[strum(serialize = "ISOLATED")]
    Isolated,
    #[strum(serialize = "CROSSED")]
    Crossed,
}

#[derive(Copy, Clone, Display)]
pub enum AutoCloseType {
    #[strum(serialize = "LIQUIDATION")]
    Liquidation,
    #[strum(serialize = "ADL")]
    ADL,
}

#[derive(Copy, Clone, Display)]
pub enum PriceMatchType {
    #[strum(serialize = "OPPONENT")]
    Opponent,
    #[strum(serialize = "OPPONENT_5")]
    Opponent5,
    #[strum(serialize = "OPPONENT_10")]
    Opponent10,
    #[strum(serialize = "OPPONENT_20")]
    Opponent20,
    #[strum(serialize = "QUEUE")]
    Queue,
    #[strum(serialize = "QUEUE_5")]
    Queue5,
    #[strum(serialize = "QUEUE_10")]
    Queue10,
    #[strum(serialize = "QUEUE_20")]
    Queue20,
}
impl FromStr for PriceMatchType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "OPPONENT" => Ok(PriceMatchType::Opponent),
            "OPPONENT_5" => Ok(PriceMatchType::Opponent5),
            "OPPONENT_10" => Ok(PriceMatchType::Opponent10),
            "OPPONENT_20" => Ok(PriceMatchType::Opponent20),
            "QUEUE" => Ok(PriceMatchType::Queue),
            "QUEUE_5" => Ok(PriceMatchType::Queue5),
            "QUEUE_10" => Ok(PriceMatchType::Queue10),
            "QUEUE_20" => Ok(PriceMatchType::Queue20),
            _ => Err("Invalid PriceMatchType".to_string()),
        }
    }
}

#[derive(Copy, Clone, Display)]
#[strum(serialize_all = "UPPERCASE")]
pub enum Side {
    Buy,
    Sell,
}

impl FromStr for Side {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "BUY" => Ok(Side::Buy),
            "SELL" => Ok(Side::Sell),
            _ => Err("Invalid Side".to_string()),
        }
    }
}

#[derive(Copy, Clone, Display)]
pub enum OrderType {
    #[strum(serialize = "LIMIT")]
    Limit,
    #[strum(serialize = "MARKET")]
    Market,
    #[strum(serialize = "STOP")]
    Stop,
    #[strum(serialize = "TAKE_PROFIT")]
    TakeProfit,
    #[strum(serialize = "STOP_MARKET")]
    StopMarket,
    #[strum(serialize = "TAKE_PROFIT_MARKET")]
    TakeProfitMarket,
    #[strum(serialize = "TRAILING_STOP_MARKET")]
    TrailingStopMarket,
}

impl FromStr for OrderType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "LIMIT" => Ok(OrderType::Limit),
            "MARKET" => Ok(OrderType::Market),
            "STOP" => Ok(OrderType::Stop),
            "TAKE_PROFIT" => Ok(OrderType::TakeProfit),
            "STOP_MARKET" => Ok(OrderType::StopMarket),
            "TAKE_PROFIT_MARKET" => Ok(OrderType::TakeProfitMarket),
            "TRAILING_STOP_MARKET" => Ok(OrderType::TrailingStopMarket),
            _ => Err("Invalid OrderType".to_string()),
        }
    }
}

#[derive(Copy, Clone, Display)]
#[strum(serialize_all = "UPPERCASE")]
pub enum TimeInForceType {
    Gtc,
    Ioc,
    Fok,
}
impl FromStr for TimeInForceType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GTC" => Ok(TimeInForceType::Gtc),
            "IOC" => Ok(TimeInForceType::Ioc),
            "FOK" => Ok(TimeInForceType::Fok),
            _ => Err("Invalid TimeInForceType".to_string()),
        }
    }
}

#[derive(Copy, Clone, Display)]
#[strum(serialize_all = "UPPERCASE")]
pub enum NewOrderResponseType {
    Ack,
    Result,
    Full,
}

impl FromStr for NewOrderResponseType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ACK" => Ok(NewOrderResponseType::Ack),
            "RESULT" => Ok(NewOrderResponseType::Result),
            "FULL" => Ok(NewOrderResponseType::Full),
            _ => Err("Invalid NewOrderResponseType".to_string()),
        }
    }
}