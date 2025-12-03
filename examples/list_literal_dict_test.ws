# Test list literal followed by dictionary

def main() -> int {
    print_str("Creating list literal...")
    numbers: list[int] = [1, 2, 3]

    print_str("Iterating list...")
    for num in numbers {
        print_int(num)
    }

    print_str("Creating dictionary...")
    scores: dict[str, int] = {}
    scores["Test"] = 42

    val: int = scores["Test"]
    print_int(val)

    print_str("Done!")
    return 0
}
