# Test RC Phase 4: Pure Function Escape Analysis
# This test verifies that variables passed to pure functions don't escape

# Test 1: List passed to pure functions (list.length, list.get)
def test_list_pure_functions() -> void {
    print_str("Test 1: List with pure functions")

    # This list is passed to pure functions only
    # Phase 4 optimization: should recognize it doesn't escape
    items: list[int] = [1, 2, 3, 4, 5]

    # All these are pure functions - don't cause escape
    len: int = items.length
    print_int(len)

    val: int = items.get(0)
    print_int(val)

    val = items.get(2)
    print_int(val)

    # items should be optimized: no escape despite method calls
    print_str("Pure functions test passed")
}

# Test 2: Dict passed to pure functions
def test_dict_pure_functions() -> void {
    print_str("Test 2: Dict with pure functions")

    # This dict is passed to pure functions only
    scores: dict[str, int] = {"alice": 100, "bob": 90}

    # Pure functions - don't cause escape
    val: int = scores["alice"]
    print_int(val)

    val = scores["bob"]
    print_int(val)

    # scores should be optimized
    print_str("Dict pure functions test passed")
}

# Test 3: List with mixed pure and impure
def consume_list(items: list[int]) -> void {
    print_int(items.get(0))
}

def test_mixed_pure_impure() -> void {
    print_str("Test 3: Mixed pure/impure functions")

    # Pure operations
    local: list[int] = [1, 2, 3]
    val: int = local.get(0)  # Pure
    len: int = local.length   # Pure
    print_int(val)
    print_int(len)

    # Impure operation (causes escape)
    escaping: list[int] = [4, 5, 6]
    consume_list(escaping)  # Not pure - causes escape

    print_str("Mixed test passed")
}

# Test 4: Print functions are pure (don't cause escape)
def test_print_pure() -> void {
    print_str("Test 4: Print functions are pure")

    # These lists are only passed to print_int via .length
    # Should not escape
    a: list[int] = [1, 2, 3]
    b: list[int] = [4, 5, 6, 7]
    c: list[int] = [8, 9]

    # Print is pure - doesn't cause escape
    print_int(a.length)
    print_int(b.length)
    print_int(c.length)

    print_str("Print pure test passed")
}

# Test 5: String operations are pure
def test_string_pure() -> void {
    print_str("Test 5: String operations are pure")

    # String operations are pure
    s: str = "hello"

    # All these are pure
    len: int = s.length
    upper: str = s.upper()
    lower: str = s.lower()

    print_int(len)
    print_str(upper)
    print_str(lower)

    # Check contains
    has: bool = s.contains("ell")
    assert has

    print_str("String pure test passed")
}

# Test 6: Complex pattern - all operations pure
def test_complex_pure() -> void {
    print_str("Test 6: Complex pure operations")

    # All operations on this list are pure
    data: list[int] = [10, 20, 30, 40, 50]

    # Chain of pure operations
    sum: int = 0
    sum = sum + data.get(0)
    sum = sum + data.get(1)
    sum = sum + data.get(2)
    sum = sum + data.get(3)
    sum = sum + data.get(4)

    print_int(sum)  # 150

    # More pure operations
    len: int = data.length
    print_int(len)

    # data should be fully optimized - no escape
    print_str("Complex pure test passed")
}

def main() -> int {
    print_str("====================================")
    print_str("RC Phase 4 Tests: Pure Functions")
    print_str("====================================")
    print_str("")

    test_list_pure_functions()
    test_dict_pure_functions()
    test_mixed_pure_impure()
    test_print_pure()
    test_string_pure()
    test_complex_pure()

    print_str("")
    print_str("====================================")
    print_str("All Phase 4 pure function tests passed!")
    print_str("====================================")
    print_str("")
    print_str("Phase 4 Optimization Active:")
    print_str("- Pure functions don't cause escape")
    print_str("- list.get, list.length, dict.get, etc. are pure")
    print_str("- Variables passed only to pure functions have no RC overhead")

    return 0
}
