# Print demo - showing console output

def factorial(n: int) -> int {
    if n <= 1 {
        return 1
    }
    return n * factorial(n - 1)
}

def is_even(n: int) -> bool {
    return (n % 2) == 0
}

def main() -> int {
    print_str("=== WadeScript Print Demo ===")
    print_str("")

    # Basic printing
    print_str("Integers:")
    print_int(42)
    print_int(-17)
    print_int(0)
    print_str("")

    print_str("Floats:")
    print_float(3.14159)
    print_float(-2.5)
    print_float(0.0)
    print_str("")

    print_str("Booleans:")
    print_bool(True)
    print_bool(False)
    print_str("")

    # Calculations
    print_str("Calculations:")
    x: int = 10
    y: int = 5

    print_str("10 + 5 =")
    print_int(x + y)

    print_str("10 * 5 =")
    print_int(x * y)

    print_str("10 > 5 is:")
    print_bool(x > y)
    print_str("")

    # Function results
    print_str("Function calls:")
    print_str("factorial(5) =")
    print_int(factorial(5))

    print_str("is_even(10) =")
    print_bool(is_even(10))

    print_str("is_even(7) =")
    print_bool(is_even(7))
    print_str("")

    print_str("=== Demo Complete ===")

    return 0
}
