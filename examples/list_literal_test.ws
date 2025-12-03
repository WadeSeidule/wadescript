# Test list literals to identify the issue

def main() -> int {
    print_str("Creating list literal...")
    numbers: list[int] = [1, 2, 3]

    print_str("Iterating...")
    for num in numbers {
        print_int(num)
    }

    print_str("After iteration")

    # Try to access length
    len: int = numbers.length
    print_int(len)

    print_str("Done!")
    return 0
}
