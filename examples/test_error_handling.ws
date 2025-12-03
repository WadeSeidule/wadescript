# Comprehensive test of error handling improvements
# This demonstrates:
# 1. Colored, informative error messages
# 2. Stack traces showing function call history
# 3. Bounds checking for lists
# 4. Key validation for dictionaries

def test_list_bounds() -> void {
    print_str("=== Testing List Bounds Checking ===")
    numbers: list[int] = [10, 20, 30]

    print_str("Accessing valid index:")
    val: int = numbers[1]
    print_int(val)

    print_str("Accessing out of bounds (this will error):")
    bad_val: int = numbers[5]
    print_int(bad_val)
}

def test_dict_keys() -> void {
    print_str("=== Testing Dictionary Key Validation ===")
    scores: dict[str, int] = {}
    scores["Alice"] = 95

    print_str("Accessing existing key:")
    score: int = scores["Alice"]
    print_int(score)

    print_str("Accessing non-existent key (this will error):")
    bad_score: int = scores["Bob"]
    print_int(bad_score)
}

def main() -> int {
    print_str("Error Handling Test Suite")
    print_str("")

    # Uncomment one of these to see different error types:
    test_list_bounds()
    # test_dict_keys()

    return 0
}
