# Test if getting string.length but not using it triggers bug

def test_strings() -> void {
    print_str("String operations:")
    text: str = "Hello WadeScript"
    len: int = text.length  # Get length but don't use it in f-string
    print_str("Text: Hello WadeScript")
    print_str("Length: 16")

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
