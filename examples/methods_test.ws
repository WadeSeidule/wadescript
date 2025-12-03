# Test if string methods + iterations cause issues

def test_string_methods_and_iteration() -> void {
    print_str("String methods:")
    text: str = "Hello WadeScript"

    upper: str = text.upper()
    lower: str = text.lower()
    print_str(upper)
    print_str(lower)

    has_wade: bool = text.contains("Wade")
    if has_wade {
        print_str("Found Wade")
    }

    print_str("String iteration:")
    for char in "ABC" {
        print_str(char)
    }
    print_str("")
}

def test_list_iteration() -> void {
    print_str("List iteration:")
    numbers: list[int] = range(5)
    print_str(f"Length: {numbers.length}")

    for num in numbers {
        print_int(num)
    }
    print_str("")
}

def test_dict() -> void {
    print_str("Dictionary:")
    scores: dict[str, int] = {}
    scores["Alice"] = 95
    val: int = scores["Alice"]
    print_int(val)
    print_str("")
}

def main() -> int {
    test_string_methods_and_iteration()
    test_list_iteration()
    test_dict()

    print_str("Success!")
    return 0
}
