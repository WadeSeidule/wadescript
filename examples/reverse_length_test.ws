# Test reversing which .length is in f-string

def test_strings() -> void {
    print_str("String:")
    text: str = "Hello"
    # Using .length DIRECTLY in f-string
    print_str(f"Text length: {text.length}")

    for char in "ABC" {
        print_str(char)
    }
    print_str("")
}

def test_lists() -> void {
    print_str("List:")
    numbers: list[int] = range(5)
    # Using .length in a variable first
    len: int = numbers.length
    print_str(f"List length: {len}")

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
