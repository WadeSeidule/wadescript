# Test using .length directly in f-strings

def test_strings() -> void {
    print_str("String:")
    text: str = "Hello"
    # Using variables in f-string
    len: int = text.length
    print_str(f"Text: {text}")
    print_str(f"Length: {len}")

    for char in "ABC" {
        print_str(char)
    }
    print_str("")
}

def test_lists() -> void {
    print_str("List:")
    numbers: list[int] = range(5)
    # Using .length DIRECTLY in f-string (like in length_test.ws)
    print_str(f"Initial list length: {numbers.length}")

    for num in numbers {
        print_int(num)
    }
    print_str("")
}

def test_dict() -> void {
    print_str("Dictionary:")
    scores: dict[str, int] = {}
    scores["Test"] = 42
    val: int = scores["Test"]
    print_int(val)
    print_str("")
}

def main() -> int {
    test_strings()
    test_lists()
    test_dict()
    print_str("Success!")
    return 0
}
