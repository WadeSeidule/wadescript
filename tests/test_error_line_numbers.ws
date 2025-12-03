# Test that runtime errors include line numbers

def main() -> int {
    numbers: list[int] = [1, 2, 3]

    # This should error on line 7 with line number in message
    val: int = numbers[10]

    return 0
}
