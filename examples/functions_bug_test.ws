# Test with separate functions

def test_strings() -> void {
    print_str("String operations:")
    text: str = "Hello"
    len: int = text.length
    print_str(f"Length: {len}")

    for char in "ABC" {
        print_str(char)
    }
    print_str("")
}

def test_lists() -> void {
    print_str("List operations:")
    numbers: list[int] = range(5)
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
    print_str("Done!")
    return 0
}
