from strategy.template import Template

class BollStrategy(Template):
    def run(self):
        self.engine.subscribe_tick("m2501", self.on_tick)
        #self.context['history_data'] = self.engine.subscribe_kline("m2501", "2m", 100)
        # do something to determine if it needs to continuely work one timing selection.
        #self.context['data_bars'] = self.engine.subscribe_kline("m2501", "1m", 100, self.on_1_minute)

    def on_tick(self, tick):
        print(self.name, "tick", tick)

    def on_1_minute(self, bars, bar):
        print(self.name, "kline", bar)
        self.handle_sell(bar)
        self.handle_buy(bar)

    def handle_buy(self, bar: dict) -> None:
        '''
        self.engine.place_order(self.name, {
            'symbol' : 'm2501',
            'order_ref' : '',
            'offset' : 'OPEN',
            'order_type' : 'FAK',
            'exchange_id' : 'DCZ',
            'volume_total' : 10,
            'direction' : 'LONG',
            'limit_price' : 2500.0,
        })
        '''
        pass

    def handle_sell(self, bar: dict) -> None:
        pass
