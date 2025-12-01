# Simple array test

def main() -> int {
    # Create a list literal
    numbers: list[int] = [1, 2, 3, 4, 5]

    # Access by index
    first: int = numbers[0]
    print_int(first)

    # Get length
    len: int = numbers.length
    print_int(len)

    return 0
}
