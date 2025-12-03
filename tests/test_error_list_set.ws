# Test list assignment error with line number

def main() -> int {
    numbers: list[int] = [10, 20, 30]

    # This should error on line 7 with line number
    numbers[100] = 999

    return 0
}
