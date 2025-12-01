# Test: Basic types and operations

def main() -> int {
    # Integer operations
    a: int = 10
    b: int = 5
    print_int(a + b)    # 15
    print_int(a - b)    # 5
    print_int(a * b)    # 50
    print_int(a / b)    # 2
    print_int(a % b)    # 0

    # Float operations
    x: float = 3.5
    y: float = 2.0
    print_float(x + y)  # 5.5
    print_float(x * y)  # 7.0

    # Boolean operations
    print_bool(True)    # True
    print_bool(False)   # False
    print_bool(10 > 5)  # True
    print_bool(10 < 5)  # False

    # String
    print_str("Hello")  # Hello

    return 0
}
