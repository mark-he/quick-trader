//You should create your own entity to extend the Class below

class Display:
    def __str__(self):
        return f"{self.inner}"


class Position(Display):
    def __init__(self, inner: dict):
        self.inner = inner

    def cost(self) -> float:
        return self.inner['cost']

    def amount(self) -> float:
        return self.inner['amount']

    def side(self) -> str:
        return self.inner['side']

    def position_side(self) -> str:
        return self.inner['positionSide']


class Order(Display):
    def __init__(self, inner: dict):
        self.inner = inner

    def status(self) -> str:
        return self.inner['status']

    def client_order_id(self) -> str:
        return self.inner['clientOrderId']

    def traded(self) -> float:
        return self.inner['traded']

    def side(self) -> str:
        return self.inner['side']

    def datetime(self) -> str:
        pass


class Account(Display):
    def __init__(self, inner: dict):
        self.inner = inner

    def balance(self) -> float:
        return self.inner['balance']
