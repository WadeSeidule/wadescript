# Test private method access enforcement
class Account {
    balance: int
    pin: int

    def init(self: Account) {
        print_str("Account created")
    }

    def get_balance(self: Account) -> int {
        return self.balance
    }

    def _verify_pin(self: Account, pin_attempt: int) -> bool {
        return self.pin == pin_attempt
    }
}

def main() -> int {
    acc: Account = Account(1000, 1234)

    # This should work: calling public method
    print_int(acc.get_balance())

    # This should fail at type checking: calling private method
    acc._verify_pin(1234)

    return 0
}
