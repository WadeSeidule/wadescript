# Lists Demo - Showing What Works Today!

def main() -> int {
    print_str("=== WadeScript Lists Demo ===")
    print_str("")

    # Create empty lists
    print_str("Creating empty lists...")
    numbers: list[int] = []
    names: list[str] = []
    scores: list[float] = []

    # Check their lengths
    print_str("Empty list lengths:")
    print_int(numbers.length)  # 0
    print_int(names.length)    # 0
    print_int(scores.length)   # 0
    print_str("")

    # Type safety works!
    print_str("Type safety: list[int] is not list[str]")
    print_str("(Compiler checks this at compile time!)")
    print_str("")

    # Memory allocation works
    print_str("Lists use dynamic memory allocation")
    print_str("Each list is 24 bytes + element storage")
    print_str("")

    # Test with variable length
    a: list[int] = []
    b: list[int] = []
    total: int = a.length + b.length
    print_str("Sum of two empty list lengths:")
    print_int(total)  # 0
    print_str("")

    print_str("=== Demo Complete ===")
    print_str("")
    print_str("See LISTS.md for implementation details!")

    return 0
}
