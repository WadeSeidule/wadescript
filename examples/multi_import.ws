# Multiple imports demo

import "math_lib"
import "lib/list_utils"

def main() -> int {
    print_str("=== Multiple Imports Demo ===")
    print_str("")

    # Use math functions
    print_str("Math functions:")
    print_int(math_lib.add(5, 10))
    print_int(math_lib.square(7))
    print_str("")

    # Use list functions
    print_str("List functions:")
    numbers: list[int] = [1, 2, 3, 4, 5]

    print_str("Sum:")
    print_int(list_utils.sum_list(numbers))

    print_str("Max:")
    print_int(list_utils.max_in_list(numbers))

    print_str("Count evens:")
    print_int(list_utils.count_evens(numbers))

    print_str("")
    print_str("=== Demo Complete ===")
    return 0
}
