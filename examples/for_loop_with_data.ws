# For Loop Test - Testing iteration with actual data

def main() -> int {
    print_str("=== For Loop with Data Test ===")
    print_str("")

    # Test with populated list
    print_str("Test: Iterating over list[1, 2, 3, 4, 5]")
    numbers: list[int] = [1, 2, 3, 4, 5]

    print_str("List length:")
    print_int(numbers.length)
    print_str("")

    print_str("Elements:")
    for num in numbers {
        print_int(num)
    }
    print_str("")

    print_str("=== Test Complete ===")
    return 0
}
