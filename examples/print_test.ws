# Test printing to console

def main() -> int {
    # Print integers
    print_int(42)
    print_int(100)

    # Print floats
    print_float(3.14159)
    print_float(2.718)

    # Print strings
    print_str("Hello, WadeScript!")
    print_str("Printing works!")

    # Print booleans
    print_bool(True)
    print_bool(False)

    # Test with calculations
    x: int = 10
    y: int = 32
    print_int(x + y)

    # Test with comparison
    result: bool = x > 5
    print_bool(result)

    return 0
}
