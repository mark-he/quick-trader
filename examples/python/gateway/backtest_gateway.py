import ctypes
import datetime
import os
from ctypes import *
import threading


class CData:
    def to_dict(self):
        result = {}
        for field, field_type in self._fields_:
            value = getattr(self, field)
            if field_type == c_char_p:
                result[field] = value.decode()
            else:
                result[field] = value
        return result

    def from_dict(self, data):
        for field, field_type in self._fields_:
            value = data[field]
            if field_type == c_char_p:
                value = value.encode('utf-8')
            setattr(self, field, value)
        return self

class CKLine(Structure, CData):
    _fields_ = [
        ("datetime", c_char_p),
        ("open", c_double),
        ("high", c_double),
        ("low", c_double),
        ("close", c_double),
        ("volume", c_int),
        ("turnover", c_double)
    ]

class COrder(Structure, CData):
    _fields_ = [
        ("order_ref", c_char_p),
        ("direction", c_char_p),
        ("offset", c_char_p),
        ("price", c_double),
        ("volume_total_original", c_int),
        ("submit_status", c_char_p),
        ("order_type", c_char_p),
        ("sys_id", c_char_p),
        ("status", c_char_p),
        ("volume_traded", c_int),
        ("volume_total", c_int),
        ("status_msg", c_char_p),
        ("symbol", c_char_p),
        ("request_id", c_int),
        ("invest_unit_id", c_char_p)
    ]

class CTrade(Structure, CData):
    _fields_ = [
        ("order_ref", c_char_p),
        ("trade_id", c_char_p),
        ("sys_id", c_char_p),
        ("direction", c_char_p),
        ("offset", c_char_p),
        ("price", c_double),
        ("volume", c_int),
        ("datetime", c_char_p),
        ("symbol", c_char_p)
    ]

class CStatus(Structure, CData):
    _fields_ = [
        ("code", c_int),
        ("message", c_char_p)
    ]

class COrderInsert(Structure, CData):
    _fields_ = [
        ("symbol", c_char_p),
        ("order_ref", c_char_p),
        ("offset", c_char_p),
        ("order_type", c_char_p),
        ("exchange_id", c_char_p),
        ("volume_total", c_int),
        ("direction", c_char_p),
        ("limit_price", c_double)
    ]

class COrderAction(Structure, CData):
    _fields_ = [
        ("symbol", c_char_p),
        ("action_ref", c_int),
        ("exchange_id", c_char_p),
        ("sys_id", c_char_p)
    ]

class CTradeConfig(Structure, CData):
    _fields_ = [
        ("front_addr", c_char_p),
        ("broker_id", c_char_p),
        ("auth_code", c_char_p),
        ("app_id", c_char_p),
        ("user_id", c_char_p),
        ("password", c_char_p)
    ]

class CTick(Structure, CData):
    _fields_ = [
        ("symbol", c_char_p),
        ("datetime", c_char_p),
        ("trading_day", c_char_p),
        ("open", c_double),
        ("high", c_double),
        ("low", c_double),
        ("close", c_double),
        ("volume", c_int),
        ("turnover", c_double),
        ("open_interest", c_double),
        ("last_price", c_double),
        ("bid_price1", c_double),
        ("bid_price2", c_double),
        ("bid_price3", c_double),
        ("bid_price4", c_double),
        ("bid_price5", c_double),
        ("bid_volume1", c_int),
        ("bid_volume2", c_int),
        ("bid_volume3", c_int),
        ("bid_volume4", c_int),
        ("bid_volume5", c_int),
        ("ask_price1", c_double),
        ("ask_price2", c_double),
        ("ask_price3", c_double),
        ("ask_price4", c_double),
        ("ask_price5", c_double),
        ("ask_volume1", c_int),
        ("ask_volume2", c_int),
        ("ask_volume3", c_int),
        ("ask_volume4", c_int),
        ("ask_volume5", c_int)
    ]
subs = {}
subs_lock = threading.Lock()

traders = {}
traders_lock = threading.Lock()

@CFUNCTYPE(None, c_char_p, POINTER(CTick))
def tick_callback(sub_id, result):
    data = result.contents.to_dict()
    func, _ = subs.get(sub_id.decode())
    func(data)

@CFUNCTYPE(None, c_char_p, POINTER(CKLine))
def kline_callback(sub_id, result):
    data = result.contents.to_dict()
    func, init_data = subs.get(sub_id.decode())
    init_data.append(data)
    func(init_data, data)

@CFUNCTYPE(None, c_char_p, POINTER(COrder))
def order_callback(unit_id, result):
    data = result.contents.to_dict()
    on_order, on_trade, on_status = traders.get(unit_id.decode())
    on_order(data)

@CFUNCTYPE(None, c_char_p, POINTER(CTrade))
def trade_callback(unit_id, result):
    data = result.contents.to_dict()
    on_order, on_trade, on_status = traders.get(unit_id.decode())
    on_trade(data)

@CFUNCTYPE(None, c_char_p, POINTER(CStatus))
def status_callback(unit_id, result):
    data = result.contents.to_dict()
    on_order, on_trade, on_status = traders.get(unit_id.decode())
    on_status(data)

class BacktestGateway:
    def __init__(self) -> None:
        self.last_sub_id = 0
        self.rust_lib = CDLL("D:/workspace/projects/quick-trader/target/release/quick_trader_ctp.dll")
        self.rust_lib.subscribe_kline.argtypes = [c_char_p, c_char_p, c_char_p, CFUNCTYPE(None,c_char_p, POINTER(CKLine))]
        self.rust_lib.subscribe_kline.restype = None

        self.rust_lib.subscribe_tick.argtypes = [c_char_p, c_char_p, CFUNCTYPE(None,c_char_p, POINTER(CTick))]
        self.rust_lib.subscribe_tick.restype = None

        self.rust_lib.subscribe_trade.argtypes = [c_char_p, CFUNCTYPE(None,c_char_p, POINTER(COrder)), CFUNCTYPE(None,c_char_p, POINTER(CTrade)), CFUNCTYPE(None,c_char_p, POINTER(CStatus))]
        self.rust_lib.subscribe_trade.restype = None

        self.rust_lib.place_order.argtypes = [c_char_p, POINTER(COrderInsert)]
        self.rust_lib.place_order.restype = c_int

        self.rust_lib.cancel_order.argtypes = [c_char_p, POINTER(COrderAction)]
        self.rust_lib.cancel_order.restype = c_int

        self.rust_lib.init_backtest.argtypes = []
        self.rust_lib.init_backtest.restype = None

        self.rust_lib.init_ctp.argtypes = [POINTER(CTradeConfig)]
        self.rust_lib.init_ctp.restype = None

        self.rust_lib.start.argtypes = []
        self.rust_lib.start.restype = None

    def init(self) -> None:
        self.rust_lib.init_backtest()

    def start(self) -> None:
        self.rust_lib.start()

    def subscribe_tick(self, symbol: str, func=None):
        if func:
            symbol =  c_char_p(symbol.encode('utf-8'))
            with subs_lock:
                self.last_sub_id += 1
                subs[str(self.last_sub_id)] = (func, None)
            sub_id = c_char_p(str(self.last_sub_id).encode('utf-8'))
            self.rust_lib.subscribe_tick(sub_id, symbol, tick_callback)

    def subscribe_kline(self, symbol: str, duration: str = "1m", init_count = 100, func = None):
        init_data = self.load_data(symbol, duration, init_count)

        if func:
            symbol =  c_char_p(symbol.encode('utf-8'))
            duration =  c_char_p(duration.encode('utf-8'))
            with subs_lock:
                self.last_sub_id += 1
                subs[str(self.last_sub_id)] = (func, init_data)
            sub_id =  c_char_p(str(self.last_sub_id).encode('utf-8'))
            self.rust_lib.subscribe_kline(sub_id, symbol, duration, kline_callback)
        return init_data

    def load_data(self, symbol: str, exchange: str, duration: str = "1d", count: int = 100) -> list:
        return []

    def connect_trade(self, unit_id: str, on_order = None, on_trade = None, on_status = None):
        with traders_lock:
            traders[unit_id] = (on_order, on_trade, on_status)

        unit_id = c_char_p(unit_id.encode('utf-8'))
        self.rust_lib.subscribe_trade(unit_id, order_callback, trade_callback, status_callback)

    def place_order(self, unit_id: str, order: dict):
        order_insert = COrderInsert().from_dict(order)

        unit_id = c_char_p(unit_id.encode('utf-8'))
        result = self.rust_lib.place_order(unit_id, byref(order_insert))

    def cancel_order(self, unit_id: str, order: dict):
        order_action = COrderAction().from_dict(order)

        unit_id = c_char_p(unit_id.encode('utf-8'))
        result = self.rust_lib.cancel_order(unit_id, byref(order_action))

