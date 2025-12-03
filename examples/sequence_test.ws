# Test running multiple features in sequence

def test_strings() -> void {
    print_str("=== Strings ===")
    text: str = "Hello"
    upper: str = text.upper()
    print_str(upper)
    print_str("")
}

def test_lists() -> void {
    print_str("=== Lists ===")
    numbers: list[int] = range(3)
    for num in numbers {
        print_int(num)
    }
    print_str("")
}

def test_dicts() -> void {
    print_str("=== Dictionaries ===")
    scores: dict[str, int] = {}
    scores["Alice"] = 95
    alice_score: int = scores["Alice"]
    print_str(f"Score: {alice_score}")
    print_str("")
}

def main() -> int {
    test_strings()
    test_lists()
    test_dicts()
    print_str("All done!")
    return 0
}
