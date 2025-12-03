# Test pop from empty list error

def main() -> int {
    numbers: list[int] = []

    print_str("Trying to pop from empty list:")
    val: int = numbers.pop()  # This should trigger error
    print_int(val)

    return 0
}
