# Import Demo - Using imported functions

import "math_lib"

def main() -> int {
    print_str("=== Import Demo ===")
    print_str("")

    # Use imported functions
    print_str("math_lib.add(10, 5):")
    print_int(math_lib.add(10, 5))

    print_str("math_lib.multiply(6, 7):")
    print_int(math_lib.multiply(6, 7))

    print_str("math_lib.square(8):")
    print_int(math_lib.square(8))

    print_str("math_lib.is_even(10):")
    print_bool(math_lib.is_even(10))

    print_str("math_lib.is_even(7):")
    print_bool(math_lib.is_even(7))

    return 0
}
