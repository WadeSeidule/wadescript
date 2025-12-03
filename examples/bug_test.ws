# Minimal test case for the bug

def main() -> int {
    print_str("Creating list...")
    numbers: list[int] = range(5)

    print_str("Getting length...")
    len: int = numbers.length

    print_str("Using length in f-string...")
    print_str(f"Length: {len}")

    print_str("Creating dictionary...")
    scores: dict[str, int] = {}

    print_str("Done!")
    return 0
}
