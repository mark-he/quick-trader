import ctypes
from ctypes import *
import threading
import json
from dataclasses import asdict
from utils.logger_factory import logger

assign_lock = threading.Lock()
unit_lock = {}
ticks = {}
klines = {}
events = {}


def init_unit_lock(unit_id):
    with assign_lock:
        unit_lock[unit_id] = threading.Lock()


@CFUNCTYPE(None, c_char_p, c_char_p)
def tick_callback(unit_id, result):
    data = json.loads(result.decode('utf-8'))
    lock = unit_lock[unit_id.decode('utf-8')]
    with lock:
        func = ticks.get(unit_id.decode('utf-8'))
        if func:
            func(data)


@CFUNCTYPE(None, c_char_p, c_char_p)
def kline_callback(unit_id, result):
    data = json.loads(result.decode('utf-8'))
    lock = unit_lock[unit_id.decode('utf-8')]
    with lock:
        func, init_data, init_count = klines.get(unit_id.decode('utf-8'))
        if len(init_data) == 0:
            init_data.append(data)
        elif data['datetime'] > init_data[-1]['datetime']:
            init_data.append(data)
            if len(init_data) > init_count:
                del init_data[0]
            if func:
                func(init_data, data)
        elif data['datetime'] == init_data[-1]['datetime']:
            init_data[-1] = data
            if func:
                func(init_data, data)
        else:
            logger.info("kline_callback >>> ignore")


@CFUNCTYPE(None, c_char_p, c_char_p, c_char_p)
def event_callback(unit_id, _type, result):
    _type = _type.decode('utf-8')
    data = json.loads(result.decode('utf-8'))
    lock = unit_lock[unit_id.decode('utf-8')]
    with lock:
        on_order_func, on_position_func = events.get(unit_id.decode('utf-8'))
        if _type == "ORDER":
            if on_order_func:
                on_order_func(data)
        else:
            if _type == "POSITION":
                if on_position_func:
                    on_position_func(data)


class Gateway:
    def __init__(self, lib_path: str) -> None:
        self.last_unit_id = 0
        self.rust_lib = CDLL(lib_path)
        self.rust_lib.subscribe_kline.argtypes = [c_char_p, c_char_p, c_char_p, c_int,CFUNCTYPE(None, c_char_p, c_char_p)]
        self.rust_lib.subscribe_kline.restype = c_void_p

        self.rust_lib.subscribe_tick.argtypes = [c_char_p, c_char_p, CFUNCTYPE(None, c_char_p, c_char_p)]
        self.rust_lib.subscribe_tick.restype = c_void_p

        self.rust_lib.init_symbol_trade.argtypes = [c_char_p, c_char_p, c_char_p, CFUNCTYPE(None, c_char_p, c_char_p, c_char_p)]
        self.rust_lib.init_symbol_trade.restype = c_void_p

        self.rust_lib.init.argtypes = [c_char_p, c_char_p, c_char_p]
        self.rust_lib.init.restype = c_void_p

        self.rust_lib.start.argtypes = []
        self.rust_lib.start.restype = c_void_p

        self.rust_lib.get_server_ping.argtypes = []
        self.rust_lib.get_server_ping.restype = c_void_p

        self.rust_lib.close.argtypes = []
        self.rust_lib.close.restype = c_void_p

        self.rust_lib.new_order.argtypes = [c_char_p, c_char_p]
        self.rust_lib.new_order.restype = c_void_p

        self.rust_lib.cancel_order.argtypes = [c_char_p, c_char_p]
        self.rust_lib.cancel_order.restype = c_void_p

        self.rust_lib.cancel_orders.argtypes = [c_char_p]
        self.rust_lib.cancel_orders.restype = c_void_p

        self.rust_lib.get_positions.argtypes = [c_char_p]
        self.rust_lib.get_positions.restype = c_void_p

        self.rust_lib.get_account.argtypes = [c_char_p]
        self.rust_lib.get_account.restype = c_void_p

    def handle_data(self, service_result: dict):
        if service_result['errorCode'] != 0:
            raise Exception(service_result['message'])
        return service_result.get('data')

    def init(self, exchange: str, mode: str, config):
        exchange = c_char_p(exchange.encode('utf-8'))
        mode = c_char_p(mode.encode('utf-8'))
        json_str = json.dumps(asdict(config))
        config = c_char_p(json_str.encode('utf-8'))
        result = self.rust_lib.init(exchange, mode, config)
        json_str = ctypes.cast(result, ctypes.POINTER(ctypes.c_char_p)).contents.value.decode('utf-8')
        return self.handle_data(json.loads(json_str))

    def start(self):
        result = self.rust_lib.start()
        json_str = ctypes.cast(result, ctypes.POINTER(ctypes.c_char_p)).contents.value.decode('utf-8')
        return self.handle_data(json.loads(json_str))

    def close(self):
        result = self.rust_lib.close()
        json_str = ctypes.cast(result, ctypes.POINTER(ctypes.c_char_p)).contents.value.decode('utf-8')
        return self.handle_data(json.loads(json_str))

    def get_server_ping(self):
        result = self.rust_lib.get_server_ping()
        json_str = ctypes.cast(result, ctypes.POINTER(ctypes.c_char_p)).contents.value.decode('utf-8')
        return self.handle_data(json.loads(json_str))

    def new_order(self, unit_id: str, symbol, order):
        symbol = c_char_p(symbol.encode('utf-8'))
        json_str = json.dumps(asdict(order))
        order = c_char_p(json_str.encode('utf-8'))
        result = self.rust_lib.new_order(symbol, order)
        json_str = ctypes.cast(result, ctypes.POINTER(ctypes.c_char_p)).contents.value.decode('utf-8')
        return self.handle_data(json.loads(json_str))

    def cancel_order(self, unit_id: str, symbol: str, order_id: str):
        symbol = c_char_p(symbol.encode('utf-8'))
        order_id = c_char_p(order_id.encode('utf-8'))
        result = self.rust_lib.cancel_order(symbol, order_id)
        json_str = ctypes.cast(result, ctypes.POINTER(ctypes.c_char_p)).contents.value.decode('utf-8')
        return self.handle_data(json.loads(json_str))

    def cancel_orders(self, unit_id: str, symbol: str):
        symbol = c_char_p(symbol.encode('utf-8'))
        result = self.rust_lib.cancel_orders(symbol)
        json_str = ctypes.cast(result, ctypes.POINTER(ctypes.c_char_p)).contents.value.decode('utf-8')
        return self.handle_data(json.loads(json_str))

    def get_account(self, unit_id: str, symbol: str):
        symbol = c_char_p(symbol.encode('utf-8'))
        result = self.rust_lib.get_account(symbol)
        json_str = ctypes.cast(result, ctypes.POINTER(ctypes.c_char_p)).contents.value.decode('utf-8')
        return self.handle_data(json.loads(json_str))

    def get_positions(self, unit_id: str, symbol: str):
        symbol = c_char_p(symbol.encode('utf-8'))
        result = self.rust_lib.get_positions(symbol)
        json_str = ctypes.cast(result, ctypes.POINTER(ctypes.c_char_p)).contents.value.decode('utf-8')
        return self.handle_data(json.loads(json_str))

    def subscribe_tick(self, unit_id: str, symbol: str, func=None):
        init_unit_lock(unit_id)

        symbol = c_char_p(symbol.encode('utf-8'))
        ticks[unit_id] = (func)

        unit_id = c_char_p(unit_id.encode('utf-8'))
        result = self.rust_lib.subscribe_tick(unit_id, symbol, tick_callback)
        json_str = ctypes.cast(result, ctypes.POINTER(ctypes.c_char_p)).contents.value.decode('utf-8')
        return self.handle_data(json.loads(json_str))

    def subscribe_kline(self, unit_id: str, symbol: str, interval: str = "1m", init_count=100, func=None):
        init_unit_lock(unit_id)

        symbol = c_char_p(symbol.encode('utf-8'))
        interval = c_char_p(interval.encode('utf-8'))
        init_data = []
        klines[unit_id] = (func, init_data, init_count)

        unit_id = c_char_p(unit_id.encode('utf-8'))
        result = self.rust_lib.subscribe_kline(unit_id, symbol, interval, init_count, kline_callback)
        json_str = ctypes.cast(result, ctypes.POINTER(ctypes.c_char_p)).contents.value.decode('utf-8')
        data = self.handle_data(json.loads(json_str))
        init_data.extend(data)
        return init_data

    def init_symbol_trade(self, unit_id: str, symbol: str, config, on_order_func=None, on_position_func=None):
        init_unit_lock(unit_id)

        symbol = c_char_p(symbol.encode('utf-8'))
        events[unit_id] = (on_order_func, on_position_func)

        json_str = json.dumps(asdict(config))
        config = c_char_p(json_str.encode('utf-8'))
        unit_id = c_char_p(unit_id.encode('utf-8'))
        result = self.rust_lib.init_symbol_trade(unit_id, symbol, config, event_callback)
        json_str = ctypes.cast(result, ctypes.POINTER(ctypes.c_char_p)).contents.value.decode('utf-8')
        return self.handle_data(json.loads(json_str))
