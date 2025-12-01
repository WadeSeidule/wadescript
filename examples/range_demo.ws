# Range Function Demo - Python-style numeric iteration

def main() -> int {
    print_str("=== WadeScript range() Demo ===")
    print_str("")

    # Example 1: Basic range
    print_str("Example 1: range(5)")
    for i in range(5) {
        print_int(i)
    }
    print_str("")

    # Example 2: Larger range
    print_str("Example 2: range(10)")
    for i in range(10) {
        print_int(i)
    }
    print_str("")

    # Example 3: Computing sum using range
    print_str("Example 3: Sum of 0 to 9")
    sum: int = 0
    for i in range(10) {
        sum = sum + i
    }
    print_str("Sum:")
    print_int(sum)  # Should print 45 (0+1+2+...+9)
    print_str("")

    # Example 4: Multiplication table
    print_str("Example 4: Multiples of 7")
    for i in range(5) {
        result: int = i * 7
        print_int(result)
    }
    print_str("")

    # Example 5: range(0) produces empty list
    print_str("Example 5: range(0) - no output")
    for i in range(0) {
        print_int(i)
    }
    print_str("Done with empty range")
    print_str("")

    print_str("=== Demo Complete ===")
    return 0
}
