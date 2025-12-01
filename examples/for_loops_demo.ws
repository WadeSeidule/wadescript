# For Loops Demo - Python-style iteration in WadeScript

def main() -> int {
    print_str("=== WadeScript For Loops Demo ===")
    print_str("")

    # Example 1: Basic iteration
    print_str("Example 1: Basic iteration")
    numbers: list[int] = [10, 20, 30, 40, 50]
    for n in numbers {
        print_int(n)
    }
    print_str("")

    # Example 2: Computing sum
    print_str("Example 2: Computing sum")
    values: list[int] = [1, 2, 3, 4, 5]
    sum: int = 0
    for value in values {
        sum = sum + value
    }
    print_str("Sum:")
    print_int(sum)  # Should print 15
    print_str("")

    # Example 3: Finding maximum
    print_str("Example 3: Finding maximum")
    scores: list[int] = [45, 92, 67, 88, 73]
    max: int = 0
    for score in scores {
        if score > max {
            max = score
        }
    }
    print_str("Maximum score:")
    print_int(max)  # Should print 92
    print_str("")

    # Example 4: Counting elements
    print_str("Example 4: Counting elements > 50")
    count: int = 0
    for s in scores {
        if s > 50 {
            count = count + 1
        }
    }
    print_str("Count:")
    print_int(count)  # Should print 4
    print_str("")

    # Example 5: Empty list
    print_str("Example 5: Empty list (no output)")
    empty: list[int] = []
    for item in empty {
        print_int(item)  # This won't execute
    }
    print_str("Done with empty list")
    print_str("")

    print_str("=== Demo Complete ===")
    return 0
}
