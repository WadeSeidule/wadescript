# Test if string iteration + list iteration causes issues

def test_string_iteration() -> void {
    print_str("String iteration:")
    for char in "ABC" {
        print_str(char)
    }
    print_str("")
}

def test_list_iteration() -> void {
    print_str("List iteration:")
    numbers: list[int] = range(3)
    for num in numbers {
        print_int(num)
    }
    print_str("")
}

def test_dict_after() -> void {
    print_str("Dictionary test:")
    scores: dict[str, int] = {}
    scores["Test"] = 42
    val: int = scores["Test"]
    print_int(val)
    print_str("")
}

def main() -> int {
    # Test with both iterations
    test_string_iteration()
    test_list_iteration()
    test_dict_after()

    print_str("Success!")
    return 0
}
