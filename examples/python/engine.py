from gateway.backtest_gateway import BacktestGateway

class Engine:
    gateway: BacktestGateway

    def __init__(self) -> None:
        self.gateway: BacktestGateway = BacktestGateway()

    def init(self):
        self.gateway.init()

    def start(self):
        self.gateway.start()

    def subscribe_kline(self, symbol: str, duration: str = "1m", init_count: int = 100, func = None) -> list:
        return self.gateway.subscribe_kline(symbol, duration, init_count, func)

    def subscribe_tick(self, symbol: str, func = None):
        return self.gateway.subscribe_tick(symbol, func)

    def connect_trade(self, unit_id: str, on_order = None, on_trade = None, on_status = None):
        self.gateway.connect_trade(unit_id, on_order, on_trade, on_status)

    def place_order(self, unit_id: str, order: dict):
        self.gateway.place_order(unit_id, order)

