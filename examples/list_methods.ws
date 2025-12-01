# List Methods Demo - push, pop, get

def main() -> int {
    print_str("=== List Methods Demo ===")
    print_str("")

    # Create empty list and use push
    print_str("Test 1: push() method")
    numbers: list[int] = []
    print_str("Initial length:")
    print_int(numbers.length)

    numbers.push(10)
    numbers.push(20)
    numbers.push(30)

    print_str("After 3 pushes, length:")
    print_int(numbers.length)
    print_str("")

    # Test get() method
    print_str("Test 2: get() method")
    print_str("Element at index 0:")
    val: int = numbers.get(0)
    print_int(val)

    print_str("Element at index 1:")
    val = numbers.get(1)
    print_int(val)

    print_str("Element at index 2:")
    val = numbers.get(2)
    print_int(val)
    print_str("")

    # Test pop() method
    print_str("Test 3: pop() method")
    last: int = numbers.pop()
    print_str("Popped value:")
    print_int(last)

    print_str("Length after pop:")
    print_int(numbers.length)
    print_str("")

    # Build a list dynamically
    print_str("Test 4: Building list with push")
    squares: list[int] = []
    for i in range(5) {
        result: int = i * i
        squares.push(result)
    }

    print_str("Squares length:")
    print_int(squares.length)

    print_str("Squares values:")
    for sq in squares {
        print_int(sq)
    }
    print_str("")

    print_str("=== Demo Complete ===")
    return 0
}
