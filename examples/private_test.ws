# Test private member access enforcement
class Account {
    balance: int
    _pin: int

    def init(self: Account) {
        print_str("Account created")
    }

    def get_balance(self: Account) -> int {
        return self.balance
    }

    def _verify_pin(self: Account, pin: int) -> bool {
        return self._pin == pin
    }
}

def main() -> int {
    acc: Account = Account(1000, 1234)

    # This should work: accessing public field
    print_int(acc.balance)

    # This should fail at type checking: accessing private field
    print_int(acc._pin)

    return 0
}
