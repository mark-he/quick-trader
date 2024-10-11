from engine import Engine


class Template:
    def __init__(self, engine: Engine, name: str, symbol: str) -> None:
        self.engine = engine
        self.name = name
        self.symbol = symbol
        self.context = {}
        self.engine.connect_trade(name, self.on_order, self.on_trade, self.on_status)
        print("python init ended")

    def on_order(self, order):
        print(order)

    def on_trade(self, trade):
        print(trade)

    def on_status(self, status):
        print(status)