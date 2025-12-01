# For Loop Test - Testing iteration over lists

def main() -> int {
    print_str("=== For Loop Test ===")
    print_str("")

    # Test with empty list
    print_str("Test 1: Empty list")
    numbers: list[int] = []
    for num in numbers {
        print_int(num)  # Should not print anything
    }
    print_str("Empty list iteration complete")
    print_str("")

    print_str("=== Test Complete ===")
    return 0
}
