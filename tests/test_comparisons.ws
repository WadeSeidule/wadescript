# Test: Comparison and logical operators

def main() -> int {
    # Integer comparisons
    print_bool(10 == 10)  # True
    print_bool(10 != 5)   # True
    print_bool(10 > 5)    # True
    print_bool(10 < 5)    # False
    print_bool(10 >= 10)  # True
    print_bool(10 <= 10)  # True

    # Logical operators
    print_bool(True and True)    # True
    print_bool(True and False)   # False
    print_bool(False or True)    # True
    print_bool(False or False)   # False
    print_bool(not True)         # False
    print_bool(not False)        # True

    # Combined conditions
    x: int = 15
    print_bool(x > 10 and x < 20)  # True
    print_bool(x < 10 or x > 20)   # False

    # Negation
    a: int = 5
    print_int(-a)  # -5

    return 0
}
