# Import Demo - Using imported functions

import "math_lib"

def main() -> int {
    print_str("=== Import Demo ===")
    print_str("")

    # Use imported functions
    print_str("add(10, 5):")
    print_int(add(10, 5))

    print_str("multiply(6, 7):")
    print_int(multiply(6, 7))

    print_str("square(8):")
    print_int(square(8))

    print_str("is_even(10):")
    print_bool(is_even(10))

    print_str("is_even(7):")
    print_bool(is_even(7))

    return 0
}
