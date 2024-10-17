use strum::Display;

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
#[derive(Copy, Clone, Display)]
#[strum(serialize_all = "UPPERCASE")]
pub enum Side {
    Buy,
    Sell,
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

#[derive(Copy, Clone, Display)]
#[strum(serialize_all = "UPPERCASE")]
pub enum TimeInForceType {
    Gtc,
    Ioc,
    Fok,
}

#[derive(Copy, Clone, Display)]
#[strum(serialize_all = "UPPERCASE")]
pub enum NewOrderResponseType {
    Ack,
    Result,
    Full,
}
