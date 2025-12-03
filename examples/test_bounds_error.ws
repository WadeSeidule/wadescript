# Test that list bounds checking produces good error messages

def main() -> int {
    numbers: list[int] = [1, 2, 3]

    print_str("Accessing valid index 0:")
    val: int = numbers[0]
    print_int(val)

    print_str("Accessing out of bounds index 10:")
    bad_val: int = numbers[10]  # This should trigger error
    print_int(bad_val)

    return 0
}
