from engine import Engine
from strategy.boll_strategy import BollStrategy
from strategy.boll_strategy2 import BollStrategy2
import time

def start() -> None:
    engine: Engine = Engine()
    engine.init()

    boll_strategy = BollStrategy(engine, "Boll_Strategy_m2501", "m2501")
    boll_strategy.run()
    '''
    boll_strategy2 = BollStrategy2(engine, "Boll_Strategy2_m2501", "m2501")
    boll_strategy2.run()
    '''
    engine.start()
    while True:
        time.sleep(1)

if __name__ == '__main__':
    start()