# Exact recreation of the bug sequence

def main() -> int {
    # String operations
    print_str("String operations:")
    text: str = "Hello"
    len: int = text.length
    print_str(f"Length: {len}")

    # String iteration
    for char in "ABC" {
        print_str(char)
    }

    # List operations
    print_str("List operations:")
    numbers: list[int] = range(5)
    len2: int = numbers.length
    print_str(f"List length: {len2}")

    # List iteration
    for num in numbers {
        print_int(num)
    }

    # Dictionary creation
    print_str("Dictionary:")
    scores: dict[str, int] = {}
    scores["Test"] = 42
    val: int = scores["Test"]
    print_int(val)

    print_str("Done!")
    return 0
}
