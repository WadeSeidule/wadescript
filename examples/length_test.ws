# Test with .length property and f-strings

def test_string_with_length() -> void {
    print_str("String with .length:")
    text: str = "Hello WadeScript"
    len: int = text.length
    print_str(f"Text: {text}")
    print_str(f"Length: {len}")

    upper: str = text.upper()
    lower: str = text.lower()
    print_str(f"Uppercase: {upper}")
    print_str(f"Lowercase: {lower}")

    has_wade: bool = text.contains("Wade")
    if has_wade {
        print_str("Found 'Wade' in text")
    }

    for char in "ABC" {
        print_str(char)
    }
    print_str("")
}

def test_list_with_length() -> void {
    print_str("List with .length:")
    numbers: list[int] = range(5)
    print_str(f"Initial list length: {numbers.length}")

    print_str("List elements:")
    for num in numbers {
        print_int(num)
    }
    print_str("")
}

def test_dict() -> void {
    print_str("Dictionary:")
    scores: dict[str, int] = {}
    scores["Alice"] = 95
    scores["Bob"] = 87

    alice_score: int = scores["Alice"]
    bob_score: int = scores["Bob"]

    print_str(f"Alice's score: {alice_score}")
    print_str(f"Bob's score: {bob_score}")
    print_str("")
}

def main() -> int {
    print_str("Testing...")
    print_str("")

    test_string_with_length()
    test_list_with_length()
    test_dict()

    print_str("Success!")
    return 0
}
