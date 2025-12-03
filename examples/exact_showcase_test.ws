# Exact copy of showcase functions

def demonstrate_strings() -> void {
    print_str("=== String Features ===")

    # String properties
    text: str = "Hello WadeScript"
    # Removed f-strings with text variable
    print_str("Text: Hello WadeScript")
    print_str("Length: 16")

    # String methods - temporarily removed
    # upper: str = text.upper()
    # lower: str = text.lower()
    # print_str(f"Uppercase: {upper}")
    # print_str(f"Lowercase: {lower}")

    # String contains - temporarily removed
    # has_wade: bool = text.contains("Wade")
    # has_python: bool = text.contains("Python")

    # if has_wade {
    #     print_str("Found 'Wade' in text")
    # }
    # if not has_python {
    #     print_str("'Python' not found in text")
    # }

    # String iteration
    print_str("Iterating over 'ABC':")
    for char in "ABC" {
        print_str(char)
    }

    print_str("")
}

def demonstrate_lists() -> void {
    print_str("=== List Features ===")

    numbers: list[int] = range(5)
    len: int = numbers.length
    print_str(f"Initial list length: {len}")

    print_str("List elements:")
    for num in numbers {
        print_int(num)
    }

    print_str("")
}

def demonstrate_dictionaries() -> void {
    print_str("=== Dictionary Features ===")

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
    demonstrate_strings()
    demonstrate_lists()
    demonstrate_dictionaries()

    print_str("Success!")
    return 0
}
