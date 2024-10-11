from dataclasses import dataclass, field
from datetime import datetime

@dataclass
class TickData():
    symbol: str
    datetime: datetime

    name: str = ""
    volume: float = 0
    turnover: float = 0
    open_interest: float = 0
    last_price: float = 0
    last_volume: float = 0
    limit_up: float = 0
    limit_down: float = 0

    open_price: float = 0
    high_price: float = 0
    low_price: float = 0
    pre_close: float = 0

    bid_price_1: float = 0
    bid_price_2: float = 0
    bid_price_3: float = 0
    bid_price_4: float = 0
    bid_price_5: float = 0

    ask_price_1: float = 0
    ask_price_2: float = 0
    ask_price_3: float = 0
    ask_price_4: float = 0
    ask_price_5: float = 0

    bid_volume_1: float = 0
    bid_volume_2: float = 0
    bid_volume_3: float = 0
    bid_volume_4: float = 0
    bid_volume_5: float = 0

    ask_volume_1: float = 0
    ask_volume_2: float = 0
    ask_volume_3: float = 0
    ask_volume_4: float = 0
    ask_volume_5: float = 0

    localtime: datetime = None

    def __post_init__(self) -> None:
        pass

@dataclass
class BarData():
    symbol: str
    datetime: datetime

    volume: float = 0
    turnover: float = 0
    open_interest: float = 0
    open_price: float = 0
    high_price: float = 0
    low_price: float = 0
    close_price: float = 0

    def __post_init__(self) -> None:
        pass


@dataclass
class OrderData():
    symbol: str
    orderid: str
    type :str
    direction = None
    offset = None
    status: str
    datetime: datetime = None
    reference: str = ""
    price: float = 0
    volume: float = 0
    traded: float = 0

@dataclass
class TradeData():
    symbol: str
    orderid: str
    tradeid: str
    direction = None

    offset = None
    price: float = 0
    volume: float = 0
    datetime: datetime = None

@dataclass
class PositionData():

    symbol: str
    volume: float = 0
    frozen: float = 0
    price: float = 0
    pnl: float = 0
    yd_volume: float = 0



@dataclass
class AccountData():
    accountid: str
    balance: float = 0
    frozen: float = 0


