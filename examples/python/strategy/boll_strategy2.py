from strategy.template import Template

class BollStrategy2(Template):
    def run(self):
        self.context['history_data'] = self.engine.subscribe("m2501", "1d", 100)
        # do something to determine if it needs to continuely work one timing selection.
        if True:
            self.context['data_bars'] = self.engine.subscribe("m2501", "1m", 100, self.on_1_minute)

    def on_1_minute(self, bars, bar):
        print(self.name, bars, "\n")
        self.handle_sell(bar)
        self.handle_buy(bar)

    def handle_buy(self, bar: dict) -> None:
        pass

    def handle_sell(self, bar: dict) -> None:
        pass
